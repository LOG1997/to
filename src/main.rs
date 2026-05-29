mod common;
mod config;
mod run;
use crate::config::{
    action::{Action, Args},
    generate,
    handle::{
        add_config, delete_config, edit_config_file, get_about_info, list_config,
        query_command_name, search_config,
    },
};
use crate::run::assort::{CommandType, is_direct_command};
use crate::run::handle::run_command;
use anyhow::{Ok, Result};
use clap::Parser;
use colored::*;

fn main() -> Result<()> {
    generate::generate_config();
    let args = Args::parse();

    match args.action {
        Action::Add { items } => add_config(items[0].as_str(), items[1].as_str())?,
        Action::Search { query } => search_config(query[0].as_str())?,
        Action::Del { ids } => delete_config(ids.iter().map(|i| i.as_str()).collect::<Vec<_>>())?,
        Action::Ls => list_config()?,
        Action::Edit { vim } => edit_config_file(vim)?,
        Action::About => get_about_info()?,
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

            let cmd_type = is_direct_command(cmd_name);
            if cmd_type == CommandType::DirectCommand {
                return run_command(cmd_name.to_owned(), cmd_params.to_owned());
            }

            let command_result = query_command_name(cmd_name)?;

            match command_result {
                Some(value) => run_command(value, cmd_params.to_owned())?,
                None => println!("{}", "This command is invalid".red()),
            }
        }
    }
    Ok(())
}
