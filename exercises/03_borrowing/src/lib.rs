#[derive(Debug, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: i32,
}

pub fn first_word(input: &str) -> &str {
    todo!("return the first whitespace-separated word")
}

pub fn push_if_missing(items: &mut Vec<String>, item: &str) {
    todo!("push item only when it is not already present")
}

pub fn split_once_comma(input: &str) -> Option<(&str, &str)> {
    todo!("split borrowed text at the first comma")
}

pub fn add_score(player: &mut Player, points: i32) {
    todo!("increase the player's score")
}

