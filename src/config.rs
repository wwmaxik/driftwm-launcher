use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LauncherConfig {
    pub width: i32,
    pub height: i32,
    pub position: Position,
    pub margin_top: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Position {
    Top,
    Center,
    Bottom,
}

impl Default for LauncherConfig {
    fn default() -> Self {
        Self {
            width: 600,
            height: 400,
            position: Position::Top,
            margin_top: 100,
        }
    }
}

impl LauncherConfig {
    pub fn load() -> Self {
        let config_path = Self::config_path();

        if let Ok(contents) = std::fs::read_to_string(&config_path)
            && let Ok(config) = toml::from_str(&contents)
        {
            return config;
        }

        Self::default()
    }

    #[allow(dead_code)]
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path();

        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let contents = toml::to_string_pretty(self)?;
        std::fs::write(&config_path, contents)?;

        Ok(())
    }

    fn config_path() -> PathBuf {
        let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        config_dir.join("driftwm-launcher").join("config.toml")
    }
}
