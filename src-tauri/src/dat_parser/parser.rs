use std::{collections::HashMap, fs, path::Path};

use diesel::{ExpressionMethods, RunQueryDsl, insert_into, upsert::excluded};
use html_escape::decode_html_entities;
use serde::Serialize;
use winnow::{
    ModalResult, Parser, Result,
    ascii::{alphanumeric1, multispace0},
    combinator::{alt, delimited, preceded, repeat, separated, separated_pair, seq, terminated},
    error::ContextError,
    token::take_until,
};

use crate::{
    establish_connection,
    models::{Console, Game, NewGame, NewRom, Rom},
    routes::console_routes::get_console_by_name,
    schemas::{
        games::{self, console_id},
        games_table,
        roms::{self},
        roms_table,
    },
};

#[derive(Debug, PartialEq, Serialize)]
pub struct DatRom {
    pub name: String,
    pub md5: String,
    pub regions: Vec<String>,
    pub size: i32,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct DatGame {
    pub name: String,
    pub roms: Vec<DatRom>,
}

// TODO: expand with more information from name attribute, e.g. beta, bootleg, etc.
#[derive(Debug)]
struct DatNameInfo {
    pub name: String,
    pub regions: Vec<String>,
    pub beta: bool,
}

// TODO: maybe for cleaning the names of games/roms
fn get_region_string(region: &str) -> Option<&str> {
    match region {
        "USA" => Some("USA"),
        "JPN" => Some("Japan"),
        "EUR" => Some("Europe"),
        "GER" => Some("Germany"),
        "AUS" => Some("Australia"),
        "SPA" => Some("Spain"),
        "FRA" => Some("France"),
        "SWE" => Some("Sweden"),
        "ITA" => Some("Italia"),
        "SCA" => Some("Scandinavia"),
        _ => {
            println!("\nregion string error on: {}\n", region);
            None
        }
    }
}

fn value_parser<'s>(input: &mut &'s str) -> Result<&'s str> {
    delimited('"', take_until(1.., '"'), '"').parse_next(input)
}

fn attributes_parser<'s>(input: &mut &'s str) -> Result<HashMap<&'s str, &'s str>> {
    // println!("attributes: {:?}", &input);
    // println!("{:?}", &input);
    let attribute_parser = delimited(
        multispace0,
        separated_pair(alphanumeric1, "=", value_parser),
        multispace0,
    );

    let attributes: Vec<(&'s str, &'s str)> = repeat(0.., attribute_parser).parse_next(input)?;

    Ok(attributes.into_iter().collect())
}

/// Parse potential comma separated values inside parenthesis, e.g. for multi-region roms
fn name_parenthesis_parser<'s>(
    name_info: &'s mut DatNameInfo,
) -> impl FnMut(&mut &str) -> ModalResult<()> + 's {
    move |input: &mut &str| {
        // take each string, either until separator or until the closing parenthesis
        let items: Vec<&str> =
            separated(0.., alt((take_until(1.., ","), take_until(1.., ")"))), ",")
                .parse_next(input)?;

        for item in items {
            let s = item.trim();
            match s {
                "Japan" | "USA" | "Europe" => name_info.regions.push(s.to_string()),
                "Beta" => name_info.beta = true,
                _ => (),
            }
        }

        Ok(())
    }
}

// name parser: take until '(' then repeat parenthesis parser
fn name_parser(input: &mut &str) -> ModalResult<DatNameInfo> {
    // take name up to first '(' (0.. because there may be no '(')
    let name = take_until(0.., "(").parse_next(input)?;

    let mut name_info = DatNameInfo {
        name: decode_html_entities(name.trim()).to_string(),
        regions: Vec::new(),
        beta: false,
    };

    {
        // create the stateful closure that mutates name_info
        let mut paren = name_parenthesis_parser(&mut name_info);

        let parenthesis_start = preceded(take_until(0.., "("), "(");
        // let parenthesis_start = preceded(take_until(0.., "("), "(");
        let _: Vec<_> =
            repeat(0.., delimited(parenthesis_start, &mut paren, ')')).parse_next(input)?;
    }

    Ok(name_info)
}

fn rom_builder<'s>(
    attributes: HashMap<&'s str, &'s str>,
    regions: Vec<String>,
    name_info: &DatNameInfo,
) -> DatRom {
    let mut rom = DatRom {
        name: String::new(),
        md5: String::new(),
        regions,
        size: 0,
    };

    for (attribute, value) in attributes {
        match attribute {
            "name" => rom.name = decode_html_entities(value).to_string(),
            "md5" => rom.md5 = value.to_owned(),
            "size" => rom.size = value.parse().unwrap_or_default(),
            _ => (),
        }
    }

    if rom.regions.is_empty() {
        rom.regions = name_info.regions.clone()
    }

    rom
}

fn game_builder(rom: DatRom, name_info: DatNameInfo) -> DatGame {
    DatGame {
        name: name_info.name.to_string(),
        roms: vec![rom],
    }
}

/// Parses release tag of dat file and returns the region string.
/// Remark: There can be multiple release tags per game
fn get_region_from_release_parser(input: &mut &str) -> Result<String> {
    let release_start = preceded(take_until(0.., "<release"), "<release");

    let release_attributes = delimited(release_start, attributes_parser, ">").parse_next(input)?;

    release_attributes
        .iter()
        .find_map(|(attr, value)| (attr == &"region").then_some(value.to_string()))
        .ok_or(ContextError::new())
}

/// Gets all regions from the releases section/s, if available - else empty vec
fn releases_parser(input: &mut &str) -> Result<Vec<String>> {
    let mut releases_block = take_until(0.., "<rom").parse_next(input)?;
    let regions: Vec<String> = repeat(0.., get_region_from_release_parser)
        .parse_next(&mut releases_block)
        .expect("error reading regions from release block/s");

    let regions = regions
        .iter()
        .map(|region| get_region_string(region).unwrap_or("").to_string())
        .collect();

    Ok(regions)
}

fn game_parser<'s>(input: &mut &'s str) -> Result<HashMap<&'s str, &'s str>> {
    let tag_start = preceded(take_until(0.., "<game"), "<game");

    delimited(tag_start, attributes_parser, alt((">", "/>"))).parse_next(input)
}

// TODO: need to be able to parse multi-rom games
fn rom_parser<'s>(input: &mut &'s str) -> Result<HashMap<&'s str, &'s str>> {
    let tag_start = preceded(take_until(0.., "<rom"), "<rom");

    delimited(tag_start, attributes_parser, alt((">", "/>"))).parse_next(input)
}

/// Parses a single <game> entry in the DAT file
fn entry_parser(input: &mut &str) -> Result<DatGame> {
    let (game_data, regions, rom_data) =
        (game_parser, releases_parser, rom_parser).parse_next(input)?;

    let mut name_raw = *game_data.get("name").unwrap();
    let name_info = name_parser(&mut name_raw).expect("error parsing name");
    let rom = rom_builder(rom_data, regions, &name_info);

    Ok(game_builder(rom, name_info))
}

/// Parses all <game> entries in the DAT file
fn entries_parser(input: &mut &str) -> Result<Vec<DatGame>> {
    repeat(1.., entry_parser).parse_next(input)
}

fn combine_game_entries(games: &mut Vec<DatGame>) {
    games.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    let mut source_index = 0;

    for read_index in 1..(games.len() - 1) {
        // TODO: could change to fuzzy match of names, or similar, for better detection of same games, if needed
        if games[source_index].name.to_lowercase() == games[read_index].name.to_lowercase() {
            // need to split games first, because otherwise we have two mutable references for games at the append
            let (left, right) = games.split_at_mut(read_index);
            left[source_index].roms.append(&mut right[0].roms);
        } else {
            source_index += 1;
            if source_index != read_index {
                games.swap(source_index, read_index);
            }
        }
    }

    // drop all duplicates, which should have been swapped to the end of the vector
    games.truncate(source_index + 1);
}

fn write_data_to_db(console: Console, games: Vec<DatGame>) {
    let conn = &mut establish_connection();

    let new_games: Vec<NewGame> = games
        .iter()
        .map(|db_game| NewGame::from_dat(db_game, Some(console.id)))
        .collect();

    // insert games with dummy update, so new inserts always return the games' ids from db, because we need the ids
    let inserted_games: Vec<Game> = insert_into(games_table)
        .values(&new_games)
        .on_conflict((games::title, console_id))
        .do_update()
        .set(games::title.eq(games::title))
        .get_results::<Game>(conn)
        .expect("error saving games");

    let mut roms: Vec<NewRom> = Vec::new();

    for (index, game) in games.iter().enumerate() {
        for rom in &game.roms {
            roms.push(NewRom::from_dat(rom, &inserted_games[index].id));
        }
    }

    let inserted_roms: Vec<Rom> = insert_into(roms_table)
        .values(roms)
        .on_conflict((roms::title, roms::game_id))
        .do_update()
        .set(roms::title.eq(excluded(roms::title)))
        .get_results(conn)
        .expect("error saving roms");

    println!("saved {:?} roms in db", inserted_roms.len());
}

pub fn parse_file(path_string: &str) -> Result<()> {
    let path = Path::new(path_string);

    let dat = fs::read_to_string(path).expect("error trying to read dat file");
    let dat = &mut dat.as_str();

    let console = header_parser.parse_next(dat)?;
    let mut games = entries_parser(dat).expect("error while parsing dat game entries");

    combine_game_entries(&mut games);
    write_data_to_db(console, games);
    Ok(())
}

fn console_parser(input: &mut &str) -> Result<Console> {
    let (_manufacturer, mut name) =
        separated_pair(take_until(1.., "-"), "-", take_until(0.., "</name")).parse_next(input)?;

    println!("manufacturer: {:?}\n", &_manufacturer);

    let trimmed_name =
        terminated::<&str, &str, &str, ContextError, _, &str>(take_until(0.., "("), "(")
            .parse_next(&mut name)
            .unwrap_or(name)
            .trim();

    println!("name: {:?}\n", &trimmed_name);

    Ok(get_console_by_name(trimmed_name))
}

fn header_name_parser(input: &mut &str) -> Result<Console> {
    println!("header_name: {:?}\n", &input[0..1000]);

    let header_name_start = preceded(multispace0, "<name>");
    // println!("header_name2: {:?}\n", header_name_start);
    delimited(header_name_start, console_parser, "</name>").parse_next(input)
}

fn header_parser(input: &mut &str) -> Result<Console> {
    let console =
    seq!(_:take_until(1.., "<header>"), _: "<header>",_:take_until(1.., "<name>"), _:multispace0, header_name_parser, _:terminated(take_until(1.., "</header>"), "</header>"))
    .parse_next(input)?.0;

    println!("header_name: {:?}\n", &console);
    Ok(console)
}

/* example parts of dat file

  <game name="ActRaiser (Europe)">
      <description>ActRaiser (Europe)</description>
      <release name="ActRaiser (Europe)" region="EUR"></release>
      <rom name="ActRaiser (Europe).sfc" size="1048576" crc="09097b2b" md5="9b36075b53dec1a506b1f9334e670c63" sha1="b76621e0b9d882c8b8463203f5423ca7d45cc5bf" status="verified"></rom>
  </game>

  <game name="Secret of Mana (Europe) (Rev 1)">
    <description>Secret of Mana (Europe) (Rev 1)</description>
    <release name="Secret of Mana (Europe) (Rev 1)" region="AUS"></release>
    <release name="Secret of Mana (Europe) (Rev 1)" region="EUR"></release>
    <rom name="Secret of Mana (Europe) (Rev 1).sfc" size="2097152" crc="de112322" md5="d273dd449b204a6eb90f611e5a72f80c" sha1="cf57dc4183c6e5aadba25019d82e61c44c0de113" status="verified"></rom>
  </game>

  <game name="Star Fox 2 (Japan) (Beta) (1994-05-13)" cloneof="Star Fox 2 (USA, Europe) (Classic Mini, Switch Online, Nintendo Leak)">
    <description>Star Fox 2 (Japan) (Beta) (1994-05-13)</description>
    <rom name="Star Fox 2 (Japan) (Beta) (1994-05-13).sfc" size="1048576" crc="d8b14e9d" md5="8286b46153f5fa21236cac29f21c7ec0" sha1="5c18b39171ace891b386345e06ff72e08b7862a1"></rom>

    example of multi rom game (only one for snes):

    <game name="Mortal Kombat (Europe) (Rev 1)">
    <description>Mortal Kombat (Europe) (Rev 1)</description>
    <release name="Mortal Kombat (Europe) (Rev 1)" region="EUR"></release>
    <rom name="Mortal Kombat (Europe) (Rev 1).sfc" size="2097152" crc="047b3d88" md5="1d348d1af28db657195f926cc0207796" sha1="2b820cf5ea310db54cef4a1c0918023fec986ee4" status="verified"></rom>
    <rom name="Mortal Kombat (Europe) (Rev 1) (Patch ROM).bin" size="32768" crc="ffdb34f7" md5="d4098651b6cc8ebd6e8ac2b38c0013ce" sha1="6526c6a75121fba961b6bdc4e4b0f76a81fc9995" status="verified"></rom>
  </game>
*/

#[cfg(test)]
mod tests {
    use html_escape::decode_html_entities;

    use super::*;

    #[test]
    fn test_new_dat_file_parse() {
        let dat_file_path = String::from(
            "DATs/Nintendo - Super Nintendo Entertainment System (20251012-045317).dat",
        );

        parse_file(&dat_file_path).expect("error parsing games from dat.");

        // println!("{:#?}", &games[0..2]);
    }

    #[test]
    fn test_value() {
        let mut input = r#""test""#;
        let output = value_parser(&mut input);

        assert_eq!("test", output.unwrap_or_default())
    }

    #[test]
    fn test_attribute() {
        let mut input = r#" name="ActRaiser (Europe)""#;
        let output = attributes_parser(&mut input).unwrap();
        assert_eq!("ActRaiser (Europe)", *output.get("name").unwrap())
    }

    #[test]
    fn test_attribute_whitespace() {
        let mut input = r#" name="test" "#;
        let output = attributes_parser(&mut input).unwrap();
        println!("{:#?}", &output);
        assert_eq!("test", *output.get("name").unwrap())
    }

    #[test]
    fn test_game_parser() {
        let mut input = r#"
        <game name="'96 Zenkoku Koukou Soccer Senshuken (Japan)" id="0001">
		<category>Games</category>
		<description>'96 Zenkoku Koukou Soccer Senshuken (Japan)</description>
		<rom name="'96 Zenkoku Koukou Soccer Senshuken (Japan).sfc" size="1572864" crc="05fbb855" md5="3369347f7663b133ce445c15200a5afa" sha1="005ccd8362dc41491f89f31fc9326a6688300e0c" sha256="b2229302c1561f8a7081534f3f27de0f130864cc7c585730ada4be9ced36df4d"/>
	</game>"#;

        let output = entry_parser(&mut input).unwrap();

        println!("output: {:#?}", output);

        // assert_eq!("test", output.unwrap_or_default())
    }

    #[test]
    fn test_game_without_release() {
        let mut input = r#"
        <game name="Star Fox 2 (Japan) (Beta) (1994-05-13)" cloneof="Star Fox 2 (USA, Europe) (Classic Mini, Switch Online, Nintendo Leak)">
    <description>Star Fox 2 (Japan) (Beta) (1994-05-13)</description>
    <rom name="Star Fox 2 (Japan) (Beta) (1994-05-13).sfc" size="1048576" crc="d8b14e9d" md5="8286b46153f5fa21236cac29f21c7ec0" sha1="5c18b39171ace891b386345e06ff72e08b7862a1"></rom>
    "#;

        let output = entry_parser(&mut input).unwrap();

        println!("{:#?}", output);

        // assert_eq!("test", output.unwrap_or_default())
    }

    #[test]
    fn test_game_multi_release() {
        let mut input = r#"
        <game name="Secret of Mana (Europe) (Rev 1)">
    <description>Secret of Mana (Europe) (Rev 1)</description>
    <release name="Secret of Mana (Europe) (Rev 1)" region="AUS"></release>
    <release name="Secret of Mana (Europe) (Rev 1)" region="EUR"></release>
    <rom name="Secret of Mana (Europe) (Rev 1).sfc" size="2097152" crc="de112322" md5="d273dd449b204a6eb90f611e5a72f80c" sha1="cf57dc4183c6e5aadba25019d82e61c44c0de113" status="verified"></rom>
  </game>
    "#;

        let output = entry_parser(&mut input).unwrap();

        let correct = DatGame {
            name: "Secret of Mana".to_string(),
            roms: vec![DatRom {
                name: "Secret of Mana (Europe) (Rev 1).sfc".to_string(),
                md5: "d273dd449b204a6eb90f611e5a72f80c".to_string(),
                regions: vec!["Australia".to_string(), "Europe".to_string()],
                size: 2097152,
            }],
        };

        // println!("{:#?}", output);

        assert_eq!(correct, output)
    }

    #[test]
    fn test_game_multi_release_new_format() {
        let mut input = r#"
        <game name="Secret of Mana (Europe) (Rev 1)">
            <description>Secret of Mana (Europe) (Rev 1)</description>
            <release name="Secret of Mana (Europe) (Rev 1)" region="AUS"/>
            <release name="Secret of Mana (Europe) (Rev 1)" region="EUR"/>
            <rom name="Secret of Mana (Europe) (Rev 1).sfc" size="2097152" crc="de112322" md5="d273dd449b204a6eb90f611e5a72f80c" sha1="cf57dc4183c6e5aadba25019d82e61c44c0de113" status="verified"/>
        </game>
        "#;

        let output = entry_parser(&mut input).unwrap();

        let correct = DatGame {
            name: "Secret of Mana".to_string(),
            roms: vec![DatRom {
                name: "Secret of Mana (Europe) (Rev 1).sfc".to_string(),
                md5: "d273dd449b204a6eb90f611e5a72f80c".to_string(),
                regions: vec!["Europe".to_string()],
                size: 2097152,
            }],
        };

        println!("{:#?}", output);

        assert_eq!(correct, output)
    }

    #[test]
    fn test_multiple_games() {
        let mut input = r#"
        <game name="Secret of Mana (Europe) (Rev 1)">
    <description>Secret of Mana (Europe) (Rev 1)</description>
    <release name="Secret of Mana (Europe) (Rev 1)" region="AUS"></release>
    <release name="Secret of Mana (Europe) (Rev 1)" region="EUR"></release>
    <rom name="Secret of Mana (Europe) (Rev 1).sfc" size="2097152" crc="de112322" md5="d273dd449b204a6eb90f611e5a72f80c" sha1="cf57dc4183c6e5aadba25019d82e61c44c0de113" status="verified"></rom>
  </game>
  <game name="ActRaiser (Europe)">
      <description>ActRaiser (Europe)</description>
      <release name="ActRaiser (Europe)" region="EUR"></release>
      <rom name="ActRaiser (Europe).sfc" size="1048576" crc="09097b2b" md5="9b36075b53dec1a506b1f9334e670c63" sha1="b76621e0b9d882c8b8463203f5423ca7d45cc5bf" status="verified"></rom>
  </game>
    "#;

        let output = entries_parser(&mut input).unwrap();

        let correct = vec![
            DatGame {
                name: "Secret of Mana".to_string(),
                roms: vec![DatRom {
                    name: "Secret of Mana (Europe) (Rev 1).sfc".to_string(),
                    md5: "d273dd449b204a6eb90f611e5a72f80c".to_string(),
                    regions: vec!["Australia".to_string(), "Europe".to_string()],
                    size: 2097152,
                }],
            },
            DatGame {
                name: "ActRaiser".to_string(),
                roms: vec![DatRom {
                    name: "ActRaiser (Europe).sfc".to_string(),
                    md5: "9b36075b53dec1a506b1f9334e670c63".to_string(),
                    regions: vec!["Europe".to_string()],
                    size: 1048576,
                }],
            },
        ];

        println!("{:#?}", output);

        assert_eq!(correct, output)
    }

    #[test]
    fn test_name_parser() {
        let mut input = "Secret of Mana (Europe) (Rev 1)";
        let input2 = decode_html_entities("Pop&apos;n TwinBee (USA, Europe) (Switch Online)");
        let output = name_parser(&mut input).unwrap();
        let output2 = name_parser(&mut input2.as_ref()).unwrap();

        println!("{:#?}", output);
        println!("{:#?}", output2);

        // assert_eq!(correct, output)
    }
}
