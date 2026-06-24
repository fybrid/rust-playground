pub fn normalize_title(title: String) -> String {
    title.trim().to_lowercase()
}

pub fn total_bytes(lines: Vec<String>) -> usize {
    lines.into_iter().map(|line| line.len()).sum()
}

pub fn append_suffix(words: Vec<String>, suffix: &str) -> Vec<String> {
    words
        .into_iter()
        .map(|mut word| {
            word.push_str(suffix);
            word
        })
        .collect()
}

pub fn keep_original_and_uppercase(name: String) -> (String, String) {
    let uppercase = name.to_uppercase();
    (name, uppercase)
}

