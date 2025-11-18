use diesel::{delete, dsl::insert_into, prelude::*, result::Error, update};

use crate::{
    error::AppError,
    establish_connection,
    models::{
        Console, ConsoleWithGameRoms, ConsoleWithGames, Game, console_location::NewConsoleLocation,
    },
    routes::games_routes::get_games_for_console,
    schemas::{console_locations, console_locations_table, consoles::name, consoles_table},
};

pub fn get_consoles() -> Result<Vec<Console>, AppError> {
    let connection = &mut establish_connection();

    let mut consoles = consoles_table::table
        .filter(consoles_table::id.gt(0))
        .select(Console::as_select())
        .load(connection)?;

    consoles.sort_by(|a, b| natord::compare(&a.name, &b.name));
    Ok(consoles)
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

pub fn get_console_game_roms(console_name: &str) -> Result<ConsoleWithGameRoms, AppError> {
    let conn = &mut establish_connection();

    let console = consoles_table::table
        .filter(name.eq(console_name))
        .select(Console::as_select())
        .first(conn)?;

    let games = get_games_for_console(&console.id)?;

    Ok(ConsoleWithGameRoms { console, games })
}

pub fn add_console_location(console_id: i32, location: String) -> Result<i32, AppError> {
    let conn = &mut establish_connection();

    let result: i32 = insert_into(console_locations_table)
        .values(NewConsoleLocation {
            location,
            console_id,
        })
        // .on_conflict_do_nothing()
        .returning(console_locations_table::id)
        .get_result(conn)?;

    Ok(result)
}

pub fn delete_console_location(location_id: i32) -> Result<i32, AppError> {
    let conn = &mut establish_connection();

    let deleted: i32 = delete(console_locations_table.filter(console_locations::id.eq(location_id)))
        .execute(conn)? as i32;

    Ok(deleted)
}

pub fn update_console(console: Console) -> Result<(), AppError> {
    let conn = &mut establish_connection();
    update(consoles_table).set(&console).execute(conn)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_console_game_roms() {
        let mut console = get_console_game_roms("Super Nintendo Entertainment System").unwrap();
        console.games = console.games[..10].to_vec();

        let json = serde_json::to_string_pretty(&console).unwrap();
        println!("{}", json);
    }

    #[test]
    fn test_add_and_delete_console_location() {
        let location_id = add_console_location(1, "location".to_string()).unwrap();
        println!("saved console location with id {:?}", location_id);
        let deleted_count = delete_console_location(location_id).unwrap();
        println!("deleted {:?} row", deleted_count);
    }
}
