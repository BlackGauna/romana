use crate::{
    establish_connection,
    models::{game::Game, GameWithRoms, Rom},
    schemas::games_table,
};
use diesel::prelude::*;

pub fn get_all_games() -> Vec<Game> {
    let connection = &mut establish_connection();
    let results: Vec<Game> = games_table
        .select(Game::as_select())
        .load(connection)
        .expect("Error loading games");

    results
}

pub fn get_games_for_console(console_id: &i32) -> Vec<GameWithRoms> {
    let conn = &mut establish_connection();

    let games: Vec<Game> = games_table
        .filter(games_table::console_id.eq(console_id))
        .select(Game::as_select())
        .load(conn)
        .unwrap_or_else(|_| panic!("Error loading games for console with id {}", console_id));

    let roms = Rom::belonging_to(&games)
        .select(Rom::as_select())
        .load(conn)
        .expect("error loading game roms");

    roms.grouped_by(&games)
        .into_iter()
        .zip(games)
        .map(|(roms, game)| GameWithRoms { game, roms })
        .collect::<Vec<GameWithRoms>>()
}
