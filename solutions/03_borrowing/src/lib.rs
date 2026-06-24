#[derive(Debug, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: i32,
}

pub fn first_word(input: &str) -> &str {
    input.split_whitespace().next().unwrap_or("")
}

pub fn push_if_missing(items: &mut Vec<String>, item: &str) {
    if !items.iter().any(|existing| existing == item) {
        items.push(item.to_string());
    }
}

pub fn split_once_comma(input: &str) -> Option<(&str, &str)> {
    input.split_once(',')
}

pub fn add_score(player: &mut Player, points: i32) {
    player.score += points;
}

