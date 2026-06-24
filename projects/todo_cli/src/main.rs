use std::fs;
use std::path::Path;

use todo_cli::TodoList;

fn main() {
    let path = Path::new("todos.json");
    let mut list = fs::read_to_string(path)
        .ok()
        .and_then(|content| TodoList::from_json(&content).ok())
        .unwrap_or_default();

    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("add") => {
            let title = args[2..].join(" ");
            let id = list.add(title);
            fs::write(path, list.to_json().expect("failed to serialize todos"))
                .expect("failed to write todos");
            println!("added todo {id}");
        }
        Some("complete") => {
            let id = args
                .get(2)
                .and_then(|value| value.parse().ok())
                .unwrap_or(0);
            if list.complete(id) {
                fs::write(path, list.to_json().expect("failed to serialize todos"))
                    .expect("failed to write todos");
                println!("completed todo {id}");
            } else {
                eprintln!("todo not found: {id}");
                std::process::exit(1);
            }
        }
        _ => {
            for item in list.pending() {
                println!("{}: {}", item.id, item.title);
            }
        }
    }
}
