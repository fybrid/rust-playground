use std::collections::HashMap;

use exercise_04_option_result::{
    find_user_name, parse_port, required_setting, safe_divide, ConfigError,
};

#[test]
fn parses_valid_ports() {
    assert_eq!(parse_port("8080"), Ok(8080));
    assert_eq!(parse_port("0"), Err("port must be greater than zero".to_string()));
    assert!(parse_port("not-a-number").is_err());
}

#[test]
fn divides_safely() {
    assert_eq!(safe_divide(10, 2), Some(5));
    assert_eq!(safe_divide(10, 0), None);
}

#[test]
fn finds_user_by_id() {
    let users = HashMap::from([(7, "Aki".to_string())]);
    assert_eq!(find_user_name(&users, 7), Some("Aki"));
    assert_eq!(find_user_name(&users, 8), None);
}

#[test]
fn validates_required_settings() {
    let settings = HashMap::from([
        ("host".to_string(), "localhost".to_string()),
        ("token".to_string(), "".to_string()),
    ]);

    assert_eq!(required_setting(&settings, "host"), Ok("localhost"));
    assert_eq!(
        required_setting(&settings, "token"),
        Err(ConfigError::EmptyValue("token".to_string()))
    );
    assert_eq!(
        required_setting(&settings, "port"),
        Err(ConfigError::MissingKey("port".to_string()))
    );
}

