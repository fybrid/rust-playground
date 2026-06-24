use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    MissingKey(String),
    EmptyValue(String),
}

pub fn parse_port(input: &str) -> Result<u16, String> {
    let port = input
        .parse::<u16>()
        .map_err(|error| format!("invalid port: {error}"))?;

    if port == 0 {
        Err("port must be greater than zero".to_string())
    } else {
        Ok(port)
    }
}

pub fn safe_divide(left: i32, right: i32) -> Option<i32> {
    if right == 0 {
        None
    } else {
        Some(left / right)
    }
}

pub fn find_user_name(users: &HashMap<u64, String>, id: u64) -> Option<&str> {
    users.get(&id).map(String::as_str)
}

pub fn required_setting<'a>(
    settings: &'a HashMap<String, String>,
    key: &str,
) -> Result<&'a str, ConfigError> {
    match settings.get(key) {
        Some(value) if value.is_empty() => Err(ConfigError::EmptyValue(key.to_string())),
        Some(value) => Ok(value.as_str()),
        None => Err(ConfigError::MissingKey(key.to_string())),
    }
}

