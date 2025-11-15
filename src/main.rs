use std::env;
use std::process::exit;

mod config;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        exit(1);
    }

    let config_path = config::expand_path("~/bin/rustmarks.toml");
    let mut cfg = config::load_items(&config_path);

    match args[1].as_str() {
        "list" => {
            println!("{}", cfg.items);
        }
        "add" => {
            let Some(alias) = args.get(2) else {
                eprintln!("Error: 'add' requires an alias argument");
                print_usage();
                exit(1);
            };

            let dir = env::current_dir().unwrap_or_else(|e| {
                eprintln!("Error: Could not get current directory: {}", e);
                exit(1);
            });

            let dir_str = dir.to_string_lossy().to_string();
            println!("Adding alias '{}' -> '{}'", alias, dir_str);

            cfg.items
                .insert(alias.clone(), toml::Value::String(dir_str));
            config::save_items(&config_path, &cfg);
        }
        "jump" => {
            let Some(alias) = args.get(2) else {
                eprintln!("Error: 'jump' requires an alias argument");
                print_usage();
                exit(1);
            };

            match cfg.items.get(alias).and_then(|v| v.as_str()) {
                Some(path) => println!("{}", path),
                None => {
                    eprintln!("Error: Alias '{}' not found", alias);
                    exit(1);
                }
            }
        }
        _ => {
            eprintln!("Error: Unknown command '{}'", args[1]);
            print_usage();
            exit(1);
        }
    }
}

fn print_usage() {
    eprintln!("Usage: rustmarks <command> [args]");
    eprintln!("Commands:");
    eprintln!("  list           - List all bookmarks");
    eprintln!("  add <alias>    - Add current directory as bookmark");
    eprintln!("  jump <alias>   - Print path for bookmark alias");
}
