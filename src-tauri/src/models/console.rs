use ::diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    models::{GameWithReleases, game::Game},
    schemas::consoles::*,
};

#[derive(
    Queryable, Debug, Selectable, Serialize, Deserialize, Identifiable, PartialEq, AsChangeset,
)]
#[diesel(table_name = consoles)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
pub struct Console {
    pub id: i32,
    pub name: String,
    pub abbreviation: String,
    pub manufacturer: String,
    pub in_library: bool,
}

#[derive(Serialize, Debug)]
pub struct ConsoleWithGames {
    #[serde(flatten)]
    pub console: Console,
    pub games: Vec<Game>,
}

#[derive(Serialize, Debug)]
pub struct ConsoleWithGameRoms {
    #[serde(flatten)]
    pub console: Console,
    pub games: Vec<GameWithReleases>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = consoles)]
#[diesel(treat_none_as_default_value = false)]
pub struct NewConsole {
    pub name: String,
    pub abbreviation: String,
    pub manufacturer: String,
}
