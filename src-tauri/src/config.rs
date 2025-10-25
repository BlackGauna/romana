// #![allow(unused)]

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::routes::console_routes;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    // pub rom_paths: RomPaths,
    pub rom_paths: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RomPaths {
    pub snes: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        let mut consoles = console_routes::get_consoles();
        consoles.sort_by_key(|console| console.abbreviation.clone());
        Self {
            rom_paths: consoles
                .iter()
                .map(|console| (console.abbreviation.clone(), "".to_owned()))
                .collect(),
        }
    }
}

impl AppConfig {
    /// get the config path, either local .config folder or system app config folder
    fn config_path(app: Option<&AppHandle>) -> PathBuf {
        if let Some(app_handle) = app
            && !cfg!(debug_assertions)
        {
            app_handle
                .path()
                .app_config_dir()
                .expect("Could not get config path")
                .join("config.toml")
        } else {
            Path::new(".config/config.toml").to_path_buf()
        }
    }

    pub fn load(app: Option<&AppHandle>) -> Self {
        let path = Self::config_path(app);

        let data = fs::read_to_string(&path).unwrap_or_default();
        let mut data: AppConfig = toml::from_str(&data).unwrap_or_default();

        // merge missing fields from defaults and save
        data.fill_defaults();
        Self::save(&data, app);

        data
    }

    pub fn save(&self, app: Option<&AppHandle>) {
        let path = Self::config_path(app);
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        let toml = toml::to_string_pretty(&self).unwrap();
        // sort rom_paths keys alphabetically
        let mut toml_doc = toml_edit::DocumentMut::from_str(&toml).unwrap();
        toml_doc["rom_paths"]
            .as_table_mut()
            .unwrap()
            .sort_values_by(|a, _, b, _| a.cmp(b));

        fs::write(&path, toml_doc.to_string()).unwrap();
    }

    pub fn fill_defaults(&mut self) {
        let defaults = AppConfig::default();

        for (console, path) in defaults.rom_paths {
            if !self.rom_paths.contains_key(console.as_str()) {
                self.rom_paths.insert(console, path);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let config = AppConfig::load(None);
        println!("{:#?}", config);
    }
}
