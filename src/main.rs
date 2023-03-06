use std::env;
use std::process::exit;
use toml;

mod config;

fn main() {
    let config_path: &str = "~/bin/rustmarks.toml";
    let final_path = config::expand_path(&config_path);

    let args: Vec<String> = env::args().collect();

    let args_len = args.len();
    if args_len < 2 {
        eprintln!("not enough args {:?}", args);
        exit(1);
    }

    let mut cfg = config::load_items(&final_path);

    let command = args.get(1).unwrap();
    match command.as_str() {
        "add" => {
            if args_len != 3 {
                eprintln!("wrong number or params for `add`");
                exit(1);
            }

            let alias = args.get(2).unwrap();
            println!("adding alias {}", alias);

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
        "jump" => {
            if args_len != 3 {
                eprintln!("wrong number or params for `jump`");
                exit(1);
            }

            let alias = args.get(2).unwrap();
            match cfg.items.get(alias) {
                Some(target) => {
                    println!("{}", target.as_str().unwrap());
                }
                None => {
                    println!("target dir not found for alias: {}", alias);
                }
            }
        }
        _ => {
            eprintln!("unknown command {}", command);
        }
    }
}
