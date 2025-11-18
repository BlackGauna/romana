use diesel::prelude::*;

use crate::{
    error::AppError,
    establish_connection,
    models::{Game, Release, Rom},
    schemas::*,
};

pub fn roms_for_console(console_id: &i32) -> Result<Vec<Rom>, AppError> {
    let conn = &mut establish_connection();

    let games: Vec<Game> = games_table
        .filter(games_table::console_id.eq(console_id))
        .select(Game::as_select())
        .load::<Game>(conn)?;

    let releases = Release::belonging_to(&games)
        .select(Release::as_select())
        .load(conn)?;

    Rom::belonging_to(&releases)
        .select(Rom::as_select())
        .load(conn)
        .map_err(AppError::DatabaseError)
}
