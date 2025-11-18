use std::collections::HashMap;

use crate::{
    error::AppError,
    establish_connection,
    models::{GameWithReleases, Release, ReleaseWithRoms, Rom, game::Game},
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

pub fn get_games_for_console(console_id: &i32) -> Result<Vec<GameWithReleases>, AppError> {
    let conn = &mut establish_connection();

    let games: Vec<Game> = games_table
        .filter(games_table::console_id.eq(console_id))
        .select(Game::as_select())
        .load(conn)
        .map_err(AppError::DatabaseError)?;

    let releases = Release::belonging_to(&games)
        .select(Release::as_select())
        .load(conn)?;

    let roms = Rom::belonging_to(&releases)
        .select(Rom::as_select())
        .load(conn)
        .map_err(AppError::DatabaseError)?;

    let release_roms = roms
        .grouped_by(&releases)
        .into_iter()
        .zip(releases)
        .map(|(roms, release)| ReleaseWithRoms { release, roms })
        .collect::<Vec<ReleaseWithRoms>>();

    let mut game_collection: HashMap<i32, Vec<ReleaseWithRoms>> = HashMap::new();

    for rel in release_roms {
        game_collection
            .entry(rel.release.game_id)
            .or_default()
            .push(rel);
    }

    Ok(games
        .into_iter()
        .map(|game| {
            let release = game_collection.remove(&game.id).unwrap_or_default();
            GameWithReleases {
                game,
                releases: release,
            }
        })
        .collect())
}
