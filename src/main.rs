mod common;
mod config;
mod run;
use crate::config::{
    generate,
    handle::{add_config, delete_config, list_config, query_command_name, search_config},
};
use crate::run::assort::{CommandType, match_command_type};
use crate::run::handle::run_command;
use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use std::{fs, path::Path};

#[derive(Debug, Subcommand)]
enum Action {
    Add {
        items: Vec<String>,
    },
    Search {
        query: Vec<String>,
    },
    Delete {
        ids: Vec<String>,
    },
    List,
    #[command(external_subcommand)]
    Custom(Vec<String>),
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

fn main() -> Result<()> {
    generate::generate_config();
    let args = Args::parse();

    let com_path = Path::new("./src.rs");
    if com_path.is_dir() {
        println!("is dir");
    } else if com_path.is_file() {
        println!("is file");
    }
    match args.action {
        Action::Add { items } => add_config(items[0].as_str(), items[1].as_str())?,
        Action::Search { query } => search_config(query[0].as_str())?,
        Action::Delete { ids } => {
            delete_config(ids.iter().map(|i| i.as_str()).collect::<Vec<_>>())?
        }
        Action::List => list_config()?,
        Action::Custom(args) => {
            // args 的第一个元素是命令名，后面是参数
            if args.is_empty() {
                anyhow::bail!("缺少命令名");
            }
            let cmd_name = match args.get(0) {
                Some(name) => name,
                None => {
                    return Err(anyhow::anyhow!("缺少命令名"));
                }
            };
            let cmd_params = args.get(1).map(|s| s.as_str()).unwrap_or("");
            let command_result = query_command_name(cmd_name)?;
            let cmd_type = match_command_type(cmd_name);
            if cmd_type == CommandType::DirectCommand {
                return run_command(cmd_name.to_owned(), cmd_params.to_owned());
            }
            match command_result {
                Some(value) => run_command(value, cmd_params.to_owned())?,
                None => println!("this is no command"),
            }
        }
    }
    Ok(())
}
