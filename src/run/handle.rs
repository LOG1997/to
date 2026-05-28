use crate::run::assort::{CommandType, match_command_type};
use anyhow::Result;
use std::process::{Command, Stdio};

pub fn run_command(value: String) -> Result<()> {
    let command_type = match_command_type(value.as_str());
    match command_type {
        CommandType::DirPath => {
            println!("values is {:?}", value);
            Command::new("xdg-open")
                .arg(value)
                .status()
                .expect("命令执行失败");
        }
        CommandType::FilePath => {
            Command::new("xdg-open")
                .arg(value)
                .status()
                .expect("命令执行失败");
        }
        CommandType::LocalApp => {
            Command::new(value)
                .stdout(Stdio::null()) // 丢弃标准输出
                .stderr(Stdio::null()) // 丢弃标准错误
                .status()?; // 执行并等待完成
        }
        CommandType::WebPage => {
            Command::new("xdg-open")
                .arg(value)
                .status()
                .expect("命令执行失败");
        }
        CommandType::Other => {
            println!("so soryy,i cant run your command")
        }
    }
    Ok(())
}
