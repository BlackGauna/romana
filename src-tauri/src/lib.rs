use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::{
    models::{Console, ConsoleWithGameRoms, ConsoleWithGames, GameWithRoms},
    routes::{
        console_routes::{self, get_all_consoles_with_games, get_console_with_game_roms},
        games_routes,
    },
};

pub mod dat_parser;
pub mod models;
pub mod routes;
pub mod schemas;

// TODO: refactor tauri commands
#[tauri::command]
fn get_consoles() -> Vec<Console> {
    console_routes::get_consoles()
}

#[tauri::command]
fn get_consoles_games() -> Vec<ConsoleWithGames> {
    get_all_consoles_with_games().expect("Error getting games")
}

#[tauri::command]
fn get_console_game_roms(console_name: String) -> ConsoleWithGameRoms {
    get_console_with_game_roms(&console_name)
}

#[tauri::command]
fn get_game_roms_for_console(console_id: i32) -> Vec<GameWithRoms> {
    games_routes::get_games_for_console(&console_id)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_consoles,
            get_consoles_games,
            get_console_game_roms,
            get_game_roms_for_console
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut connection = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    diesel::sql_query("PRAGMA journal_mode = WAL;")
        .execute(&mut connection)
        .expect("Failed to enable WAL mode");

    connection
}
