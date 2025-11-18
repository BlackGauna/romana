use std::{collections::HashMap, fs, path::Path, str::FromStr};

use diesel::{Connection, ExpressionMethods, RunQueryDsl, insert_into, upsert::excluded};
use html_escape::decode_html_entities;
use serde::Serialize;
use winnow::{
    ModalResult, Parser, Result,
    ascii::{alphanumeric1, multispace0},
    combinator::{alt, delimited, preceded, repeat, separated, separated_pair, seq, terminated},
    error::ContextError,
    token::{rest, take_until},
};

use crate::{
    error::AppError,
    establish_connection,
    models::{
        Console, Game, NewGame, NewRelease, NewReleaseRegion, NewRom, Region, Release, ReleaseType,
    },
    routes::console_routes::get_console_by_name,
    schemas::{
        games::{self, console_id},
        games_table, release_regions, release_regions_table,
        releases::{self},
        releases_table, roms, roms_table,
    },
};

#[derive(Debug, PartialEq, Serialize)]
pub struct DatRom {
    pub name: String,
    pub md5: String,
    pub crc: String,
    pub size: i32,
    pub release_insert_id: i32,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct DatRelease {
    pub name: String,
    pub roms: Vec<DatRom>,
    pub regions: Vec<Region>,
    pub insert_id: i32,
    pub release_type: ReleaseType,
    pub misc: String,
    pub revision: i32,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct DatGame {
    pub name: String,
}

// TODO: expand with more information from name attribute, e.g. beta, bootleg, etc.
#[derive(Debug)]
struct DatNameInfo {
    pub name: String,
    pub regions: Vec<Region>,
    pub revision: i32,
    pub release_type: ReleaseType,
    pub misc: String,
}

struct CombinedDatEntries(
    Vec<DatGame>,
    HashMap<String, Vec<DatRelease>>,
    HashMap<i32, Vec<Region>>,
    HashMap<i32, Vec<DatRom>>,
);

// TODO: maybe for cleaning the names of games/roms
fn get_region_string(region: &str) -> Region {
    match region {
        "USA" => Region::USA,
        "JPN" => Region::Japan,
        "EUR" => Region::Europe,
        "GER" => Region::Germany,
        "AUS" => Region::Australia,
        "SPA" => Region::Spain,
        "FRA" => Region::France,
        "SWE" => Region::Sweden,
        "ITA" => Region::Italia,
        "SCA" => Region::Scandinavia,
        _ => {
            println!("\nregion string error on: {}\n", region);
            Region::World
        }
    }
}

fn value_parser<'s>(input: &mut &'s str) -> Result<&'s str> {
    delimited('"', take_until(1.., '"'), '"').parse_next(input)
}

fn attributes_parser<'s>(input: &mut &'s str) -> Result<HashMap<&'s str, &'s str>> {
    let attribute_parser = delimited(
        multispace0,
        separated_pair(alphanumeric1, "=", value_parser),
        multispace0,
    );

    let attributes: Vec<(&'s str, &'s str)> = repeat(0.., attribute_parser).parse_next(input)?;

    Ok(attributes.into_iter().collect())
}

/// Parse potential comma separated values inside parenthesis in the name, e.g. for multi-region roms
fn name_parenthesis_parser<'s>(
    name_info: &'s mut DatNameInfo,
) -> impl FnMut(&mut &str) -> ModalResult<()> + 's {
    move |input: &mut &str| {
        // take each string, either until separator or until the closing parenthesis
        let items: Vec<&str> =
            separated(0.., alt((take_until(1.., ","), take_until(1.., ")"))), ",")
                .parse_next(input)?;

        // TODO: improve parsing of additional blocks and revisions of betas
        for item in items {
            let s = item.trim();
            match s {
                s if Region::from_str(s).is_ok() => {
                    name_info.regions.push(Region::from_str(s).unwrap())
                }
                s if ReleaseType::from_str(s).is_ok() => {
                    name_info.release_type = ReleaseType::from_str(s).unwrap()
                }
                // match beta with number explicitely
                s if s.contains("Beta") => {
                    name_info.release_type = ReleaseType::Beta;
                    name_info.revision = s.replace("Beta", "").trim().parse::<i32>().unwrap();
                    // Beta 1 -> revision 0, Beta 2 -> revision 1, ...
                    name_info.revision -= 1;
                }
                s if s.contains("Rev") => {
                    name_info.revision = s.split("Rev ").collect::<Vec<&str>>()[1]
                        .parse()
                        .unwrap_or_default()
                }
                _ => name_info.misc = s.to_owned(),
            }
        }

        Ok(())
    }
}

// name parser: take until '(' then repeat parenthesis parser
fn name_parser(input: &mut &str) -> ModalResult<DatNameInfo> {
    // take name up to first '(' - (0.. because there may be no '(')
    let name = take_until(0.., "(").parse_next(input)?;

    let mut name_info = DatNameInfo {
        name: decode_html_entities(name.trim()).to_string(),
        regions: Vec::new(),
        release_type: ReleaseType::Official,
        misc: "".to_owned(),
        revision: 0,
    };

    {
        // create the stateful closure that mutates name_info
        let mut paren = name_parenthesis_parser(&mut name_info);

        let parenthesis_start = preceded(take_until(0.., "("), "(");
        let _: Vec<_> =
            repeat(0.., delimited(parenthesis_start, &mut paren, ')')).parse_next(input)?;
    }

    Ok(name_info)
}

fn rom_builder<'s>(attributes: HashMap<&'s str, &'s str>) -> DatRom {
    let mut rom = DatRom {
        name: String::new(),
        md5: String::new(),
        crc: String::new(),
        size: 0,
        release_insert_id: 0,
    };

    for (attribute, value) in attributes {
        match attribute {
            "name" => rom.name = decode_html_entities(value).to_string(),
            "md5" => rom.md5 = value.to_owned(),
            "crc" => rom.crc = value.to_owned(),
            "size" => rom.size = value.parse().unwrap_or_default(),
            _ => (),
        }
    }

    rom
}

fn release_builder(roms: Vec<DatRom>, regions: Vec<Region>, name_info: DatNameInfo) -> DatRelease {
    let mut release = DatRelease {
        name: name_info.name.to_string(),
        roms,
        regions,
        insert_id: 0,
        release_type: name_info.release_type,
        misc: name_info.misc,
        revision: name_info.revision,
    };

    if release.regions.is_empty() {
        release.regions = name_info.regions.clone();
    }

    release
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

/// Gets all regions from the releases section/s, if available - else empty vec.
/// This <release> element is available in no-intro parent-clone DATs
fn releases_parser(input: &mut &str) -> Result<Vec<Region>> {
    let mut releases_block = take_until(0.., "<rom").parse_next(input)?;
    let regions: Vec<String> = repeat(0.., get_region_from_release_parser)
        .parse_next(&mut releases_block)
        .expect("error reading regions from release block/s");

    let regions = regions
        .iter()
        .map(|region| get_region_string(region))
        .collect();

    Ok(regions)
}

fn game_parser<'s>(input: &mut &'s str) -> Result<HashMap<&'s str, &'s str>> {
    let tag_start = preceded(take_until(0.., "<game"), "<game");

    delimited(tag_start, attributes_parser, alt((">", "/>"))).parse_next(input)
}

/// parses one or more <rom> blocks
fn roms_parser<'s>(input: &mut &'s str) -> Result<Vec<HashMap<&'s str, &'s str>>> {
    let mut roms_block = alt((take_until(0.., "<game"), rest)).parse_next(input)?;
    let tag_start = preceded(take_until(0.., "<rom"), "<rom");

    repeat(
        1..,
        delimited(tag_start, attributes_parser, alt((">", "/>"))),
    )
    .parse_next(&mut roms_block)
}

fn console_parser(input: &mut &str) -> Result<Console> {
    let (_manufacturer, mut name) =
        separated_pair(take_until(1.., "-"), "-", take_until(0.., "</name")).parse_next(input)?;

    // println!("manufacturer: {:?}\n", &_manufacturer);

    let trimmed_name =
        terminated::<&str, &str, &str, ContextError, _, &str>(take_until(0.., "("), "(")
            .parse_next(&mut name)
            .unwrap_or(name)
            .trim();

    // println!("name: {:?}\n", &trimmed_name);

    Ok(get_console_by_name(trimmed_name))
}

fn header_name_parser(input: &mut &str) -> Result<Console> {
    // println!("header_name: {:?}\n", &input[0..1000]);

    let header_name_start = preceded(multispace0, "<name>");
    // println!("header_name2: {:?}\n", header_name_start);
    delimited(header_name_start, console_parser, "</name>").parse_next(input)
}

fn header_parser(input: &mut &str) -> Result<Console> {
    let console =
    seq!(_:take_until(1.., "<header>"), _: "<header>",_:take_until(1.., "<name>"), _:multispace0, header_name_parser, _:terminated(take_until(1.., "</header>"), "</header>"))
    .parse_next(input)?.0;

    // println!("header_name: {:?}\n", &console);
    Ok(console)
}

/// Parses a single <game> entry in the DAT file
fn entry_parser(input: &mut &str) -> Result<DatRelease> {
    let (game_data, regions, roms_data) =
        (game_parser, releases_parser, roms_parser).parse_next(input)?;

    let mut name_raw = *game_data.get("name").unwrap();
    let name_info = name_parser(&mut name_raw).expect("error parsing name");

    let roms: Vec<DatRom> = roms_data
        .into_iter()
        .map(|rom_data| rom_builder(rom_data))
        .collect();

    Ok(release_builder(roms, regions, name_info))
}

/// Parses all <game> entries in the DAT file
fn entries_parser(input: &mut &str) -> Result<Vec<DatRelease>> {
    repeat(1.., entry_parser).parse_next(input)
}

pub fn parse_file(path_string: &str) -> Result<()> {
    let path = Path::new(path_string);

    let dat = fs::read_to_string(path).expect("error trying to read dat file");
    let dat = &mut dat.as_str();

    let console = header_parser.parse_next(dat)?;
    let releases = entries_parser(dat).expect("error while parsing dat game entries");

    let CombinedDatEntries(games, releases, release_regions, roms) = combine_game_entries(releases);

    write_data_to_db(console, games, releases, release_regions, roms).unwrap();
    Ok(())
}

fn combine_game_entries(dat_releases: Vec<DatRelease>) -> CombinedDatEntries {
    let mut games: HashMap<String, DatGame> = HashMap::new();
    let mut releases: HashMap<String, Vec<DatRelease>> = HashMap::new();
    let mut release_regions: HashMap<i32, Vec<Region>> = HashMap::new();
    let mut roms: HashMap<i32, Vec<DatRom>> = HashMap::new();

    for (index, mut release) in dat_releases.into_iter().enumerate() {
        // make the index the unique insert_id for db insertion later
        release.insert_id = i32::try_from(index).expect("could not cast usize to i32");

        roms.insert(release.insert_id, release.roms);
        release.roms = vec![];

        release_regions.insert(release.insert_id, release.regions.clone());

        let name = release.name.to_lowercase();
        let game_name = match games.entry(name.clone()) {
            std::collections::hash_map::Entry::Occupied(entry) => entry.key().to_lowercase(),
            _ => {
                games.insert(
                    name,
                    DatGame {
                        name: release.name.clone(),
                    },
                );
                release.name.to_lowercase()
            }
        };

        match releases.entry(game_name.clone()) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                entry.get_mut().push(release);
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(vec![release]);
            }
        }
    }

    CombinedDatEntries(
        games.into_values().collect(),
        releases,
        release_regions,
        roms,
    )
}

fn write_data_to_db(
    console: Console,
    games: Vec<DatGame>,
    releases: HashMap<String, Vec<DatRelease>>,
    release_regions: HashMap<i32, Vec<Region>>,
    roms: HashMap<i32, Vec<DatRom>>,
) -> Result<(), AppError> {
    let conn = &mut establish_connection();

    conn.transaction(|conn| {
        let new_games: Vec<NewGame> = games
            .iter()
            .map(|db_game| NewGame::from_dat(db_game, console.id))
            .collect();

        // insert games with dummy update, so new inserts always return the games' ids from db, because we need the ids
        let inserted_games: HashMap<String, i32> = insert_into(games_table)
            .values(&new_games)
            .on_conflict((games::title, console_id))
            .do_update()
            .set(games::title.eq(games::title))
            .get_results::<Game>(conn)?
            .into_iter()
            .map(|game| (game.title.to_lowercase(), game.id))
            .collect();

        let releases: Vec<NewRelease> = releases
            .iter()
            .flat_map(|(game_name, r)| {
                r.iter().map(|release| {
                    NewRelease::from_dat(release, inserted_games.get(game_name).unwrap())
                })
            })
            .collect();

        let inserted_releases: HashMap<i32, i32> = insert_into(releases_table)
            .values(&releases)
            .on_conflict((
                releases::title_non_null,
                releases::game_id,
                releases::revision,
                releases::parent_id_non_null,
                releases::regions_hash,
                releases::r#type,
                releases::type_misc,
            ))
            .do_update()
            .set((
                releases::revision.eq(excluded(releases::revision)),
                releases::parent_id.eq(excluded(releases::parent_id)),
                releases::insert_id.eq(excluded(releases::insert_id)),
            ))
            .get_results::<Release>(conn)?
            .into_iter()
            .map(|release| (release.insert_id, release.id))
            .collect();

        let release_regions: Vec<NewReleaseRegion> = release_regions
            .iter()
            .flat_map(|(id, roms_vec)| {
                roms_vec
                    .iter()
                    .map(|r| NewReleaseRegion::from_dat(r, inserted_releases.get(id).unwrap()))
            })
            .collect();

        insert_into(release_regions_table)
            .values(&release_regions)
            .on_conflict((release_regions::region_id, release_regions::release_id))
            .do_nothing()
            .execute(conn)?;

        let roms: Vec<NewRom> = roms
            .iter()
            .flat_map(|(id, roms_vec)| {
                roms_vec
                    .iter()
                    .map(|r| NewRom::from_dat(r, inserted_releases.get(id).unwrap()))
            })
            .collect();

        insert_into(roms_table)
            .values(roms)
            .on_conflict((roms::title, roms::release_id))
            .do_update()
            .set(roms::title.eq(excluded(roms::title)))
            .execute(conn)?;

        Ok(())
    })
    .map_err(AppError::DatabaseError)
}

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
    fn test_multi_rom() {
        let mut input = r#"
        <game name="Mortal Kombat (Europe) (Rev 1)" id="1681">
            <category>Games</category>
            <description>Mortal Kombat (Europe) (Rev 1)</description>
            <rom name="Mortal Kombat (Europe) (Rev 1).sfc" size="2097152" crc="047b3d88" md5="1d348d1af28db657195f926cc0207796" sha1="2b820cf5ea310db54cef4a1c0918023fec986ee4" sha256="8d043d680d621b198547490e25ffad4c0ce218a352a217be1ae6724fbc324ede" status="verified"/>
            <rom name="Mortal Kombat (Europe) (Rev 1) (Patch ROM).bin" size="32768" crc="ffdb34f7" md5="d4098651b6cc8ebd6e8ac2b38c0013ce" sha1="6526c6a75121fba961b6bdc4e4b0f76a81fc9995" sha256="18b775ebd6266e68915641e4f914b65ffebdbeee67280c2d105717e733ff16f6" status="verified"/>
        </game>
        <game name="Secret of Mana (Europe) (Rev 1)">
            <description>Secret of Mana (Europe) (Rev 1)</description>
            <release name="Secret of Mana (Europe) (Rev 1)" region="AUS"></release>
            <release name="Secret of Mana (Europe) (Rev 1)" region="EUR"></release>
            <rom name="Secret of Mana (Europe) (Rev 1).sfc" size="2097152" crc="de112322" md5="d273dd449b204a6eb90f611e5a72f80c" sha1="cf57dc4183c6e5aadba25019d82e61c44c0de113" status="verified"></rom>
        </game>
        "#;

        let output = entries_parser(&mut input);

        println!("{:#?}", output);
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

        let correct = DatRelease {
            name: "Secret of Mana".to_string(),
            regions: vec![Region::Australia, Region::Europe],
            roms: vec![DatRom {
                name: "Secret of Mana (Europe) (Rev 1).sfc".to_string(),
                md5: "d273dd449b204a6eb90f611e5a72f80c".to_string(),
                crc: "de112322".to_string(),
                size: 2097152,
                release_insert_id: 0,
            }],
            insert_id: 0,
            release_type: ReleaseType::Official,
            misc: "".to_owned(),
            revision: 1,
        };

        println!("{:#?}", output);
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

        let correct = DatRelease {
            name: "Secret of Mana".to_string(),
            regions: vec![Region::Australia, Region::Europe],
            roms: vec![DatRom {
                name: "Secret of Mana (Europe) (Rev 1).sfc".to_string(),
                md5: "d273dd449b204a6eb90f611e5a72f80c".to_string(),
                crc: "de112322".to_string(),
                size: 2097152,
                release_insert_id: 0,
            }],
            insert_id: 0,
            release_type: ReleaseType::Official,
            misc: "".to_owned(),
            revision: 1,
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
            DatRelease {
                name: "Secret of Mana".to_string(),
                regions: vec![Region::Australia, Region::Europe],
                roms: vec![DatRom {
                    name: "Secret of Mana (Europe) (Rev 1).sfc".to_string(),
                    md5: "d273dd449b204a6eb90f611e5a72f80c".to_string(),
                    crc: "de112322".to_string(),
                    size: 2097152,
                    release_insert_id: 0,
                }],
                insert_id: 0,
                release_type: ReleaseType::Official,
                misc: "".to_owned(),
                revision: 1,
            },
            DatRelease {
                name: "ActRaiser".to_string(),
                roms: vec![DatRom {
                    name: "ActRaiser (Europe).sfc".to_string(),
                    md5: "9b36075b53dec1a506b1f9334e670c63".to_string(),
                    crc: "09097b2b".to_string(),
                    size: 1048576,
                    release_insert_id: 0,
                }],
                regions: vec![Region::Europe],
                insert_id: 0,
                release_type: ReleaseType::Official,
                misc: "".to_owned(),
                revision: 0,
            },
        ];

        // println!("{:#?}", output);
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
