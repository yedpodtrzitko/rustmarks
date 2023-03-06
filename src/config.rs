use std::{env, fs};
use std::path::{PathBuf};
use std::process::exit;
use toml::Table;


#[derive(serde_derive::Serialize)]
#[derive(serde_derive::Deserialize)]
#[derive(Debug)]
pub struct Config {
    pub items: Table,
}


pub fn expand_path(config_path: &str) -> String {
    if config_path.starts_with('~') {
        let path_buf: PathBuf = [env::var("HOME").unwrap(), config_path[2..].to_string()].iter().collect();
        return path_buf.display().to_string();
    }

    config_path.to_string()
}

pub fn load_items(config_path: &str) -> Config {
    let contents = match fs::read_to_string(&config_path) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Could not read config file `{}`: {}", config_path.clone(), err);
            exit(1);
        }
    };

    let config: Config = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(err) => {
            eprintln!("Unable to load data from `{}`: {}", config_path.clone(), err);
            exit(1);
        }
    };

    config
}


pub fn save_items(config_path: &str, config: &Config) {
    let config_str = toml::to_string(&config).unwrap();
    _ = fs::write(config_path, &config_str);
}
