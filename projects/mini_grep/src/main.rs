use std::fs;

use mini_grep::search;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: mini_grep <query> <path>");
        std::process::exit(1);
    }

    let contents = fs::read_to_string(&args[2]).expect("failed to read input file");
    for line in search(&args[1], &contents, true) {
        println!("{line}");
    }
}
