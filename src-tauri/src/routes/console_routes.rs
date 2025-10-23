use diesel::{prelude::*, result::Error};

use crate::{
    establish_connection,
    models::{Console, ConsoleWithGameRoms, ConsoleWithGames, Game, GameWithRoms, Rom},
    schemas::{consoles::name, consoles_table},
};

pub fn get_consoles() -> Vec<Console> {
    let connection = &mut establish_connection();

    consoles_table::table
        .filter(consoles_table::id.gt(0))
        .select(Console::as_select())
        .load(connection)
        .expect("Error getting consoles")
}

pub fn get_all_consoles_with_games() -> Result<Vec<ConsoleWithGames>, Error> {
    let connection = &mut establish_connection();

    let all_consoles = consoles_table::table
        .select(Console::as_select())
        .load(connection)?;

    println!("consoles");
    println!("{:?}", all_consoles);

    let games = Game::belonging_to(&all_consoles)
        .select(Game::as_select())
        .load(connection)?;

    let games_of_consoles = games
        .grouped_by(&all_consoles)
        .into_iter()
        .zip(all_consoles)
        .map(|(games, console)| ConsoleWithGames { console, games })
        .collect::<Vec<ConsoleWithGames>>();

    println!("{:?}", games_of_consoles);

    Ok(games_of_consoles)
}

pub fn get_console_by_name(console_name: &str) -> Console {
    let conn = &mut establish_connection();

    consoles_table::table
        .filter(name.eq(console_name))
        .select(Console::as_select())
        .first(conn)
        .unwrap_or_else(|_| panic!("Error getting console with name: {}", console_name))
}

pub fn get_console_with_game_roms(console_name: &str) -> ConsoleWithGameRoms {
    let conn = &mut establish_connection();

    let console = consoles_table::table
        .filter(name.eq(console_name))
        .select(Console::as_select())
        .first(conn)
        .unwrap_or_else(|_| panic!("Error getting console with name: {}", console_name));

    let games = Game::belonging_to(&console)
        .select(Game::as_select())
        .load(conn)
        .unwrap();

    let roms = Rom::belonging_to(&games)
        .select(Rom::as_select())
        .load(conn)
        .unwrap();

    let game_roms = roms
        .grouped_by(&games)
        .into_iter()
        .zip(games)
        .map(|(roms, game)| GameWithRoms { game, roms })
        .collect::<Vec<GameWithRoms>>();

    ConsoleWithGameRoms {
        console,
        games: game_roms,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_console_game_roms() {
        let mut console = get_console_with_game_roms("Super Nintendo Entertainment System");
        console.games = console.games[..10].to_vec();

        let json = serde_json::to_string_pretty(&console).unwrap();
        println!("{}", json);
    }
}
