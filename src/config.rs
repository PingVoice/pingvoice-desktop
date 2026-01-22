use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_url")]
    pub url: String,
    #[serde(default = "default_true")]
    pub autostart: bool,
    #[serde(default = "default_true")]
    pub start_minimized: bool,
}

fn default_url() -> String {
    "https://pingvoice.io/dashboard".to_string()
}

fn default_true() -> bool {
    true
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            url: default_url(),
            autostart: true,
            start_minimized: true,
        }
    }
}

impl AppConfig {
    pub fn config_path() -> PathBuf {
        let app_data = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("io.pingvoice.app");

        fs::create_dir_all(&app_data).ok();
        app_data.join("config.json")
    }

    pub fn load() -> Self {
        let path = Self::config_path();

        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    match serde_json::from_str(&content) {
                        Ok(config) => return config,
                        Err(e) => eprintln!("Failed to parse config: {}", e),
                    }
                }
                Err(e) => eprintln!("Failed to read config: {}", e),
            }
        }

        // Create default config file if it doesn't exist
        let config = Self::default();
        config.save();
        config
    }

    pub fn save(&self) {
        let path = Self::config_path();
        if let Ok(content) = serde_json::to_string_pretty(self) {
            fs::write(&path, content).ok();
        }
    }
}
