use exercise_03_borrowing::{add_score, first_word, push_if_missing, split_once_comma, Player};

#[test]
fn returns_first_word_slice() {
    assert_eq!(first_word("rust ownership borrowing"), "rust");
    assert_eq!(first_word("single"), "single");
}

#[test]
fn pushes_only_missing_items() {
    let mut items = vec!["rust".to_string()];
    push_if_missing(&mut items, "rust");
    push_if_missing(&mut items, "cargo");
    assert_eq!(items, vec!["rust".to_string(), "cargo".to_string()]);
}

#[test]
fn splits_at_first_comma() {
    assert_eq!(split_once_comma("key,value"), Some(("key", "value")));
    assert_eq!(split_once_comma("a,b,c"), Some(("a", "b,c")));
    assert_eq!(split_once_comma("missing"), None);
}

#[test]
fn mutates_player_score() {
    let mut player = Player {
        name: "Aki".to_string(),
        score: 10,
    };
    add_score(&mut player, 7);
    assert_eq!(player.score, 17);
}

