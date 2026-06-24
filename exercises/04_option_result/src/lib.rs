use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    MissingKey(String),
    EmptyValue(String),
}

pub fn parse_port(input: &str) -> Result<u16, String> {
    todo!("parse a port from text and reject zero")
}

pub fn safe_divide(left: i32, right: i32) -> Option<i32> {
    todo!("return None for division by zero")
}

pub fn find_user_name(users: &HashMap<u64, String>, id: u64) -> Option<&str> {
    todo!("return the user's name as a borrowed string slice")
}

pub fn required_setting<'a>(
    settings: &'a HashMap<String, String>,
    key: &str,
) -> Result<&'a str, ConfigError> {
    todo!("return a non-empty setting value or a useful error")
}

