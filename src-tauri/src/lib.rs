use diesel::prelude::*;
use dotenvy::dotenv;
use std::{env, sync::Mutex};
use tauri::{Manager, State};

use crate::{
    config::AppConfig,
    models::{Console, ConsoleWithGameRoms, ConsoleWithGames, GameWithRoms},
    routes::{
        console_routes::{self, get_all_consoles_with_games, get_console_with_game_roms},
        games_routes,
    },
};

pub mod config;
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

#[tauri::command]
fn get_app_config(state: State<'_, Mutex<AppConfig>>) -> AppConfig {
    state.lock().unwrap().clone()
}

#[tauri::command]
fn save_app_config(
    new_config: AppConfig,
    state: State<'_, Mutex<AppConfig>>,
    app_handle: tauri::AppHandle,
) {
    let mut state_config = state.lock().unwrap();

    *state_config = new_config;
    state_config.save(Some(&app_handle));
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_config = AppConfig::load(Some(app.app_handle()));
            app.manage(Mutex::new(app_config));

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_consoles,
            get_consoles_games,
            get_console_game_roms,
            get_game_roms_for_console,
            get_app_config,
            save_app_config
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
