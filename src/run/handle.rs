use crate::run::assort::{CommandType, match_command_type};
use anyhow::Result;
use clap::builder::Str;
use std::process::{Command, Stdio};

pub fn run_command(value: String, param: String) -> Result<()> {
    let command_type = match_command_type(value.as_str());
    match command_type {
        CommandType::DirPath => {
            open_dir(value);
        }
        CommandType::FilePath => {
            open_file(value);
        }
        CommandType::LocalApp => {
            open_app(value)?;
        }
        CommandType::WebPage => {
            open_web_page(value, param)?;
        }
        CommandType::Other => {
            println!("so soryy,i cant run your command")
        }
    }
    Ok(())
}

fn open_web_page(url: String, parms: String) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        Command::new("start")
            .arg(url)
            .status()
            .expect("命令执行失败");
    }
    #[cfg(target_os = "linux")]
    {
        let result = insert_into_template(url.as_str(), parms.as_str());
        Command::new("xdg-open")
            .arg(result)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        Ok(())
    }
    #[cfg(target_os = "macos")]
    {
        todo!("macos open browser of url");
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

fn open_app(app_name: String) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        Command::new("vim")
            .arg(path)
            .status()
            .expect("命令执行失败");
    }
    #[cfg(target_os = "linux")]
    {
        Command::new(app_name)
            .stdout(Stdio::null()) // 丢弃标准输出
            .stderr(Stdio::null()) // 丢弃标准错误
            .spawn()?;
        Ok(())
    }
    #[cfg(target_os = "macos")]
    {
        todo!("open macos file path");
        // Command::new("vim")
        //     .arg(path)
        //     .status()
        //     .expect("命令执行失败");
    }
}
fn insert_into_template(template: &str, input: &str) -> String {
    if template.contains("{}") {
        return template.replace("{}", input);
    } else {
        return template.to_string() + input;
    }
}
