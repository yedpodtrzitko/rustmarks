use std::env;
use std::process::exit;
use toml;

mod config;

fn main() {
    const CONFIG_PATH: &str = "~/bin/rustmarks.toml";

    let args: Vec<String> = env::args().collect();

    let args_len = args.len();
    if args_len < 2 {
        println!("not enough args {:?}", args);
        return;
    }

    let mut cfg = config::load_items(CONFIG_PATH);

    let command = args.get(1).unwrap();
    match command.as_str() {
        "add" => {
            if args_len != 3 {
                println!("wrong number or params for `add`");
                return;
            }

            let alias = args.get(2).unwrap();
            println!("adding alias {}", alias);

            match env::current_dir() {
                Ok(dir) => {
                    cfg.items.insert(alias.clone(), toml::Value::String(dir.into_os_string().into_string().unwrap()));

                    config::save_items(CONFIG_PATH, &cfg);

                }
                Err(e) => {
                    println!("error adding current directory {}", e);
                    exit(1);
                }
            }
        }
        _ => {
            println!("unknown command {}", command  );
        }
    }


    println!("items {}", cfg.items);
}
