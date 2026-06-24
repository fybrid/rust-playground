use std::fs;

use file_word_count::{top_words, word_counts};

fn main() {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("usage: file_word_count <path>");
        std::process::exit(1);
    };

    let content = fs::read_to_string(path).expect("failed to read input file");
    let counts = word_counts(&content);

    for (word, count) in top_words(&counts, 10) {
        println!("{word}: {count}");
    }
}
