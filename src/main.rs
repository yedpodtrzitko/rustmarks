use std::env;
use std::process::exit;
use toml;

mod config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_path: &str = "~/bin/rustmarks.toml";
    let final_path = config::expand_path(&config_path);
    let mut cfg = config::load_items(&final_path);

    match args.get(1).map(String::as_str) {
        Some("list") => {
            println!("{}", cfg.items);
        }
        Some("add") => {
            if args.len() != 3 {
                eprintln!("Wrong number or params for `add`.");
                exit(1);
            }

            let alias = args.get(2).unwrap();
            println!("Adding alias: {}", alias);

            match env::current_dir() {
                Ok(dir) => {
                    cfg.items.insert(alias.clone(), toml::Value::String(dir.into_os_string().into_string().unwrap()));

                    config::save_items(&final_path, &cfg);
                }
                Err(e) => {
                    eprintln!("error adding current directory {}", e);
                    exit(1);
                }
            }
        }
        Some("jump") => {
            if args.len() != 3 {
                eprintln!("wrong number or params for `jump`");
                exit(1);
            }

            let alias = args.get(2).unwrap();
            match cfg.items.get(alias) {
                Some(target) => {
                    println!("{}", target.as_str().unwrap());
                }
                None => {
                    eprintln!("target dir not found for alias: {}", alias);
                    exit(1)
                }
            }
        }
        _ => {
            eprintln!("Expected arguments: 'add' / 'jump' / 'list', found: {:?}", args);
            exit(1)
        }
    }
}
