use crate::common::file_path::get_config_path;
use anyhow::Result;
use std::fs;
use toml::{Value, map::Map};

pub fn add_config(name: &str, value: &str) -> Result<()> {
    let config_path = get_config_path();
    if !config_path.exists() {
        fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    }
    let content = fs::read_to_string(config_path).unwrap_or_default();
    let mut root: Value = if content.is_empty() {
        Value::Table(Map::new())
    } else {
        toml::from_str(&content)?
    };
    let root_table = root.as_table_mut().expect("TOML 根必须是表");

    let commands_table = root_table
        .entry("commands")
        .or_insert_with(|| Value::Table(Map::new()))
        .as_table_mut()
        .expect("commands 必须是表");

    commands_table.insert(name.to_string(), Value::String(value.to_string()));

    let new_content = toml::to_string(&root)?;
    fs::write(config_path, new_content)?;
    Ok(())
}

pub fn search_config(params: &str) -> Result<()> {
    let config_path = get_config_path();
    if !config_path.exists() {
        fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    }
    let content = fs::read_to_string(config_path).unwrap_or_default();

    let mut root: Value = if content.is_empty() {
        Value::Table(Map::new())
    } else {
        toml::from_str(&content)?
    };
    let root_table = root.as_table_mut().expect("TOML 根必须是表");
    let commands_table = root_table
        .entry("commands")
        .or_insert_with(|| Value::Table(Map::new()))
        .as_table_mut()
        .expect("commands 必须是表");
    println!("cmaomsadlasmd:{:?}", commands_table);

    let results: Vec<_> = commands_table
        .iter()
        .filter(|(name, value)| {
            name.contains(params) || value.as_str().unwrap_or("").contains(params)
        })
        .collect();
    println!("结果: {:?}", results);

    Ok(())
}

pub fn list_config() -> Result<()> {
    let config_path = get_config_path();
    if !config_path.exists() {
        return Ok(());
    }
    let content = fs::read_to_string(config_path).unwrap_or_default();
    let root: Value = if content.is_empty() {
        Value::Table(Map::new())
    } else {
        toml::from_str(&content)?
    };
    let root_table = root.as_table().expect("TOML 根必须是表");
    let commands_table = root_table
        .get("commands")
        .and_then(|v| v.as_table())
        .expect("commoasdmasd");
    for (name, value) in commands_table {
        println!("{}: {}", name, value.as_str().unwrap_or(""));
    }
    Ok(())
}

pub fn delete_config(name: Vec<&str>) -> Result<()> {
    let config_path = get_config_path();
    if !config_path.exists() {
        return Ok(());
    }
    let content = fs::read_to_string(config_path).unwrap_or_default();
    let mut root: Value = if content.is_empty() {
        Value::Table(Map::new())
    } else {
        toml::from_str(&content)?
    };
    let root_table = root.as_table_mut().expect("TOML 根必须是表");
    let commands_table = root_table
        .entry("commands")
        .or_insert_with(|| Value::Table(Map::new()))
        .as_table_mut()
        .expect("commands 必须是表");

    for n in name {
        commands_table.remove(n);
        println!("删除——{}", n)
    }
    fs::write(config_path, toml::to_string(&root)?)?;
    Ok(())
}
