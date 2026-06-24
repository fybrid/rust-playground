use exercise_02_ownership::{
    append_suffix, keep_original_and_uppercase, normalize_title, total_bytes,
};

#[test]
fn normalizes_owned_title() {
    assert_eq!(normalize_title("  Rust In Production  ".to_string()), "rust in production");
}

#[test]
fn counts_bytes_by_consuming_lines() {
    let lines = vec!["abc".to_string(), "rust".to_string(), "所有".to_string()];
    assert_eq!(total_bytes(lines), 13);
}

#[test]
fn appends_suffix_to_each_word() {
    let words = vec!["owner".to_string(), "borrow".to_string()];
    assert_eq!(
        append_suffix(words, "ship"),
        vec!["ownership".to_string(), "borrowship".to_string()]
    );
}

#[test]
fn keeps_original_and_uppercase_copy() {
    assert_eq!(
        keep_original_and_uppercase("sana".to_string()),
        ("sana".to_string(), "SANA".to_string())
    );
}

