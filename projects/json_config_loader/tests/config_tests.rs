use json_config_loader::{load_config, ConfigValidationError};

#[test]
fn loads_and_validates_config() {
    let config = load_config(
        r#"{
            "app_name": "learning-api",
            "port": 8080,
            "database_url": "postgres://localhost/app"
        }"#,
    )
    .expect("valid config");

    assert_eq!(config.app_name, "learning-api");
    assert!(!config.debug);
    assert_eq!(config.validate(), Ok(()));
}

#[test]
fn rejects_invalid_config_values() {
    let config = load_config(
        r#"{
            "app_name": "",
            "port": 8080,
            "database_url": "postgres://localhost/app"
        }"#,
    )
    .expect("valid json");

    assert_eq!(config.validate(), Err(ConfigValidationError::EmptyAppName));
}
