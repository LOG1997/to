use crate::common::file_path::get_config_path;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use toml::Value;

#[derive(Serialize)]
struct ConfigData {
    name: String,
    version: String,
    commands: HashMap<String, Value>, // 动态字段
}

pub fn generate_config() {
    let config_path = get_config_path();
    if !config_path.exists() {
        fs::create_dir_all(config_path.parent().unwrap()).unwrap();
        let default_config = ConfigData {
            name: env!("CARGO_CRATE_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            commands: HashMap::new(),
        };
        let config_str = toml::to_string(&default_config).unwrap();
        fs::write(&config_path, config_str).unwrap();
    }
}
