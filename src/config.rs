use std::{env, fs};
use std::path::{Path, PathBuf};
use std::process::exit;
use toml::Table;

/*
#[derive(serde_derive::Deserialize)]
#[derive(Debug)]
pub struct Data {
    pub config: Config,
}*/

#[derive(serde_derive::Serialize)]
#[derive(serde_derive::Deserialize)]
#[derive(Debug)]
pub struct Config {
    pub items: Table,
}


fn expand_path(config_path: &str) -> String {
    if config_path.starts_with('~') {
        let path_buf: PathBuf = [env::var("HOME").unwrap(), config_path[2..].to_string()].iter().collect();
        return path_buf.display().to_string();
    }

    config_path.to_string()
}

pub fn load_items(config_path: &str) -> Config {
    let final_path = expand_path(&config_path);
    let contents = match fs::read_to_string(&final_path) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Could not read config file `{}`: {}", final_path.clone(), err);
            exit(1);
        }
    };

    let config: Config = match toml::from_str(&contents) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(err) => {
            // Write `msg` to `stderr`.
            eprintln!("Unable to load data from `{}`: {}", final_path.clone(), err);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    config
}


pub fn save_items(config_path: &str, config: &Config) {
    let final_path = expand_path(&config_path);

    let config_str = toml::to_string(&config).unwrap();
    fs::write(final_path, &config_str);
}