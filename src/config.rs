use std::fs;
use std::process::exit;
use toml::Table;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub items: Table,
}

pub fn expand_path(config_path: &str) -> String {
    if let Some(path) = config_path.strip_prefix("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return format!("{}/{}", home, path);
        }
    }
    config_path.to_string()
}

pub fn load_items(config_path: &str) -> Config {
    let contents = fs::read_to_string(config_path).unwrap_or_else(|err| {
        eprintln!("Could not read config file `{}`: {}", config_path, err);
        exit(1);
    });

    toml::from_str(&contents).unwrap_or_else(|err| {
        eprintln!("Unable to load data from `{}`: {}", config_path, err);
        exit(1);
    })
}

pub fn save_items(config_path: &str, config: &Config) {
    let config_str = toml::to_string(config).unwrap_or_else(|err| {
        eprintln!("Failed to serialize config: {}", err);
        exit(1);
    });

    fs::write(config_path, config_str).unwrap_or_else(|err| {
        eprintln!("Failed to write config file `{}`: {}", config_path, err);
        exit(1);
    });
}
