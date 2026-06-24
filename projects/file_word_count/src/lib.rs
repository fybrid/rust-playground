use std::collections::BTreeMap;

pub fn word_counts(text: &str) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();

    for word in text.split_whitespace() {
        let normalized = word
            .trim_matches(|character: char| !character.is_alphanumeric())
            .to_lowercase();

        if !normalized.is_empty() {
            *counts.entry(normalized).or_insert(0) += 1;
        }
    }

    counts
}

pub fn top_words(counts: &BTreeMap<String, usize>, limit: usize) -> Vec<(String, usize)> {
    let mut entries: Vec<_> = counts
        .iter()
        .map(|(word, count)| (word.clone(), *count))
        .collect();
    entries.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
    entries.truncate(limit);
    entries
}
