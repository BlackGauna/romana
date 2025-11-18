use diesel::prelude::*;
use dotenvy::dotenv;
use std::{env, path::Path, sync::Mutex};
use tauri::{Manager, State};
use tauri_plugin_dialog::{DialogExt, FilePath};

use crate::{
    config::AppConfig,
    models::{Console, ConsoleWithGameRoms, ConsoleWithGames, GameWithReleases},
    routes::{
        console_routes::{self, get_all_consoles_with_games, get_console_game_roms},
        games_routes,
    },
};

pub mod config;
pub mod dat_parser;
pub mod error;
pub mod models;
pub mod rom_import;
pub mod routes;
pub mod schemas;

// TODO: refactor tauri commands
#[tauri::command]
fn get_consoles() -> Vec<Console> {
    console_routes::get_consoles().expect("Error getting consoles")
}

#[tauri::command]
fn get_consoles_games() -> Vec<ConsoleWithGames> {
    get_all_consoles_with_games().expect("Error getting games")
}

#[tauri::command]
fn get_console_with_games(console_name: String) -> ConsoleWithGameRoms {
    get_console_game_roms(&console_name).unwrap()
}

#[tauri::command]
fn get_games_for_console(console_id: i32) -> Vec<GameWithReleases> {
    games_routes::get_games_for_console(&console_id).unwrap()
}

#[tauri::command]
fn get_app_config(state: State<'_, Mutex<AppConfig>>) -> AppConfig {
    state.lock().unwrap().clone()
}

#[tauri::command]
async fn choose_rom_dir(
    console_name: String,
    app_handle: tauri::AppHandle,
    state: State<'_, Mutex<AppConfig>>,
) -> Result<(), ()> {
    let path = pick_folder(&app_handle).await.unwrap();

    let mut config = state.lock().unwrap();
    config.rom_paths.insert(console_name, path.to_string());

    Ok(())
}

async fn pick_folder(app_handle: &tauri::AppHandle) -> Option<FilePath> {
    let default_dir = app_handle
        .path()
        .home_dir()
        .unwrap_or(Path::new("/").to_path_buf());

    app_handle
        .dialog()
        .file()
        .set_directory(default_dir)
        .blocking_pick_folder()
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
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_config = AppConfig::load(Some(app.app_handle()));
            app.manage(Mutex::new(app_config));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_consoles,
            get_consoles_games,
            get_console_with_games,
            get_games_for_console,
            get_app_config,
            save_app_config,
            choose_rom_dir
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
