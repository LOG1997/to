use crate::common::file_path::get_config_path;
use anyhow::Result;
use colored::*;
use std::{
    fs,
    path::{Path, absolute},
    process::Command,
};
use toml::{Value, map::Map};

pub fn add_config(name: &str, value: &str) -> Result<()> {
    let config_path = get_config_path();
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

    let mut command_value = value.to_string();
    if Path::new(value).is_dir() || Path::new(value).is_file() {
        let whole_path = absolute(value)?;
        command_value = whole_path.to_str().unwrap().to_string();
    }
    commands_table.insert(name.to_string(), Value::String(command_value));

    let new_content = toml::to_string(&root)?;
    fs::write(config_path, new_content)?;
    Ok(())
}

pub fn search_config(params: &str) -> Result<()> {
    let config_path = get_config_path();
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

    let results: Vec<_> = commands_table
        .iter()
        .filter(|(name, value)| {
            name.contains(params) || value.as_str().unwrap_or("").contains(params)
        })
        .collect();
    println!("查询结果:");
    for item in results {
        print!("{:<10}", item.0.blue());
        print!("{:^4}", "=");
        println!("{}", item.1.to_string().green())
    }

    Ok(())
}

pub fn list_config() -> Result<()> {
    let config_path = get_config_path();
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
        .expect("commands 必须是表");

    println!("查询结果:");
    for (name, value) in commands_table {
        print!("{:<10}", name.blue());
        print!("{:^4}", "=");
        println!("{}", value.to_string().green());
    }
    Ok(())
}

pub fn delete_config(names: Vec<&str>) -> Result<()> {
    let config_path = get_config_path();
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

    for n in names {
        let is_existed = commands_table.get_key_value(n).is_some();
        if !is_existed {
            println!("there is no your command: {}", n.red());
        } else {
            commands_table.remove(n);
            println!("已删除——{}", n.bright_red())
        }
    }
    fs::write(config_path, toml::to_string(&root)?)?;
    Ok(())
}

pub fn query_command_name(query_name: &str) -> Result<Option<String>> {
    let config_path = get_config_path();
    let content = fs::read_to_string(config_path).unwrap_or_default();

    let root: Value = if content.is_empty() {
        Value::Table(Map::new())
    } else {
        toml::from_str(&content)?
    };
    let root_table = root
        .as_table()
        .ok_or_else(|| anyhow::anyhow!("TOML根必须是表"))?;

    // 获取 commands 表
    let commands_table = match root_table.get("commands") {
        Some(Value::Table(table)) => table,
        _ => return Ok(None), // 如果没有 commands 表或不是表，返回 None
    };

    // 查找键名并提取字符串值
    if let Some(Value::String(value)) = commands_table.get(query_name) {
        Ok(Some(value.clone()))
    } else {
        Ok(None)
    }
}

pub fn edit_config_file(vim: bool) -> Result<()> {
    let config_path = get_config_path();
    if vim {
        Command::new("vim").arg(config_path).status()?;
    } else {
        opener::open(config_path)?;
    }
    Ok(())
}

pub fn get_about_info() -> Result<()> {
    let readme_content = include_str!("../../README.md");
    println!("name: {}", env!("CARGO_PKG_NAME").green());
    println!("version: {}", env!("CARGO_PKG_VERSION").green());
    println!();
    println!("----------------------------------------");
    println!();
    println!("{}", readme_content);
    Ok(())
}
