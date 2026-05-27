mod common;
mod config;
use crate::config::{
    generate,
    handle::{add_config, delete_config, list_config, search_config},
};
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
enum Action {
    Add { items: Vec<String> },
    Search { query: Vec<String> },
    Delete { ids: Vec<String> },
    List,
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
    match args.action {
        Action::Add { items } => add_config(items[0].as_str(), items[1].as_str())?,
        Action::Search { query } => search_config(query[0].as_str())?,
        Action::Delete { ids } => {
            delete_config(ids.iter().map(|i| i.as_str()).collect::<Vec<_>>())?
        }
        Action::List => list_config()?,
    }
    Ok(())
}
