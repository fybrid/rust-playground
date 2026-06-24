#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchConfig {
    pub query: String,
    pub path: String,
    pub case_sensitive: bool,
}

pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    if case_sensitive {
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
    } else {
        let query = query.to_lowercase();
        contents
            .lines()
            .filter(|line| line.to_lowercase().contains(&query))
            .collect()
    }
}
