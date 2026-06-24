use file_word_count::{top_words, word_counts};

#[test]
fn counts_normalized_words() {
    let counts = word_counts("Rust, rust! Cargo.");
    assert_eq!(counts.get("rust"), Some(&2));
    assert_eq!(counts.get("cargo"), Some(&1));
}

#[test]
fn returns_top_words() {
    let counts = word_counts("b a b c c c");
    assert_eq!(
        top_words(&counts, 2),
        vec![("c".to_string(), 3), ("b".to_string(), 2)]
    );
}
