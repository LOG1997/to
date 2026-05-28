use crate::run::assort::{CommandType, match_command_type};
use anyhow::Result;
use clap::builder::Str;
use std::process::{Command, Stdio};

pub fn run_command(value: String) -> Result<()> {
    let command_type = match_command_type(value.as_str());
    match command_type {
        CommandType::DirPath => {
            open_dir(value);
        }
        CommandType::FilePath => {
            open_file(value);
        }
        CommandType::LocalApp => {
            Command::new(value)
                .stdout(Stdio::null()) // 丢弃标准输出
                .stderr(Stdio::null()) // 丢弃标准错误
                .spawn()?;
        }
        CommandType::WebPage => {
            open_web_page(value);
        }
        CommandType::Other => {
            println!("so soryy,i cant run your command")
        }
    }
    Ok(())
}

fn open_web_page(url: String) {
    #[cfg(target_os = "windows")]
    {
        Command::new("start")
            .arg(url)
            .status()
            .expect("命令执行失败");
    }
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(url)
            .status()
            .expect("命令执行失败");
    }
    todo!("macos open browser of url");
    #[cfg(target_os = "macos")]
    {
        // Command::new("xdg-open")
        //     .arg(url)
        //     .status()
        //     .expect("命令执行失败");
    }
}

fn open_dir(path: String) {
    #[cfg(target_os = "windows")]
    {
        Command::new("explore")
            .arg(path)
            .status()
            .expect("命令执行失败");
    }
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(path)
            .status()
            .expect("命令执行失败");
    }
    todo!("macos open browser of url");
    #[cfg(target_os = "macos")]
    {
        // Command::new("xdg-open")
        //     .arg(path)
        //     .status()
        //     .expect("命令执行失败");
    }
}

fn open_file(path: String) {
    #[cfg(target_os = "windows")]
    {
        Command::new("vim")
            .arg(path)
            .status()
            .expect("命令执行失败");
    }
    #[cfg(target_os = "linux")]
    {
        Command::new("vim")
            .arg(path)
            .status()
            .expect("命令执行失败");
    }
    todo!("open macos file path");
    #[cfg(target_os = "macos")]
    {
        // Command::new("vim")
        //     .arg(path)
        //     .status()
        //     .expect("命令执行失败");
    }
}
