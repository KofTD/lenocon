use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default)]
pub struct Config {
    pub on_icon: String,
    pub off_icon: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            on_icon: "battery-full-charging".into(),
            off_icon: "battery-100".into(),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parsing error: {0}")]
    Parse(#[from] toml::de::Error),
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let path = std::env::var("HOME")
            .map(PathBuf::from)
            .unwrap()
            .join(".config")
            .join("lenocon-daemon")
            .join("config.toml");
        let text = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&text)?)
    }
}
