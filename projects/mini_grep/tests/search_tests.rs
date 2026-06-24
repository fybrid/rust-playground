use mini_grep::search;

#[test]
fn searches_case_sensitive() {
    let text = "Rust\ntrust\nRUST";
    assert_eq!(search("Rust", text, true), vec!["Rust"]);
}

#[test]
fn searches_case_insensitive() {
    let text = "Rust\ntrust\nRUST";
    assert_eq!(search("rust", text, false), vec!["Rust", "trust", "RUST"]);
}
