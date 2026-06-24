use std::fs;

use json_config_loader::load_config;

fn main() {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("usage: json_config_loader <config.json>");
        std::process::exit(1);
    };

    let content = fs::read_to_string(path).expect("failed to read config file");
    let config = load_config(&content).expect("failed to parse config");
    config.validate().expect("invalid config");

    println!("{} listening on {}", config.app_name, config.port);
}
