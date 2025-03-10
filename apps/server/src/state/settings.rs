use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::env;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct LovedSettings {
}

#[derive(Clone)]
pub struct LovedSettingsManager {
    settings: HashMap<String, serde_json::Value>,
    settings_path: PathBuf,
}

impl LovedSettingsManager {
    pub fn new() -> Self {
        let storage_path = env::var("SETTINGS_PATH").unwrap_or_else(|_| "storage".to_string());
        let settings_path = PathBuf::from(format!("{}/settings.json", storage_path));

        if !settings_path.exists() {
            fs::create_dir_all(&storage_path).expect("Failed to create storage directory");
            File::create(&settings_path).expect("Failed to create settings file");
        }

        let settings = if settings_path.exists() {
            let mut file = File::open(&settings_path).expect("Failed to open settings file");
            let mut contents = String::new();
            
            file.read_to_string(&mut contents).expect("Failed to read settings file");
            serde_json::from_str(&contents).unwrap_or_default()
        } else {
            HashMap::new()
        };

        LovedSettingsManager { settings, settings_path }
    }

    pub fn read(&self, key: &str) -> Option<&serde_json::Value> {
        self.settings.get(key)
    }

    pub fn read_as<T>(&self, key: &str) -> Option<T>
        where T: for<'de> Deserialize<'de>
    {
        self.settings.get(key)
            .and_then(|value| serde_json::from_value(value.clone()).ok())
    }

    pub fn update(&mut self, key: &str, value: serde_json::Value, is_admin: bool) -> Result<(), String> {
        if !is_admin {
            return Err("Unauthorized: Only admins can update settings.".to_string());
        }

        self.settings.insert(key.to_string(), value);
        self.save_settings();

        Ok(())
    }

    fn save_settings(&self) {
        let json = serde_json::to_string(&self.settings).expect("Failed to serialize settings");
        let mut file = File::create(&self.settings_path).expect("Failed to create settings file");

        file.write_all(json.as_bytes()).expect("Failed to write settings file");
    }
}

#[derive(Debug)]
pub struct ServerSettings {

}