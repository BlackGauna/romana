use ::diesel::prelude::*;
use serde::Serialize;

use crate::{
    models::{game::Game, GameWithRoms},
    schemas::consoles::*,
};

#[derive(Queryable, Debug, Selectable, Serialize, Identifiable, PartialEq)]
#[diesel(table_name = consoles)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Console {
    pub id: i32,
    pub name: String,
    pub abbreviation: String,
    pub manufacturer: String,
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
    pub games: Vec<GameWithRoms>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = consoles)]
#[diesel(treat_none_as_default_value = false)]
pub struct NewConsole {
    pub name: String,
    pub abbreviation: String,
    pub manufacturer: String,
}
