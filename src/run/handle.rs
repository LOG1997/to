use crate::run::assort::{CommandType, match_command_type};
use anyhow::Result;
use std::process::{Command, Stdio};

pub fn run_command(value: String, param: String) -> Result<()> {
    let command_type = match_command_type(value.as_str());
    match command_type {
        CommandType::DirPath => {
            open_dir(value)?;
        }
        CommandType::FilePath => {
            open_file(value)?;
        }
        CommandType::LocalApp => {
            open_app(value)?;
        }
        CommandType::WebPage => {
            open_web_page(value, param)?;
        }
        CommandType::DirectCommand => {
            opener::open(value)?;
        }
        CommandType::Other => {
            println!("so soryy,i cant run your command")
        }
    }
    Ok(())
}

fn open_web_page(url: String, parms: String) -> Result<()> {
    let result = insert_into_template(url.as_str(), parms.as_str());
    webbrowser::open(result.as_str())?;
    Ok(())
}

fn open_dir(path: String) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer.exe").arg(path).status()?;
        Ok(())
    }
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(path)
            .status()
            .expect("命令执行失败");
    }
    #[cfg(target_os = "macos")]
    {
        todo!("macos open browser of url");
    }
}

fn open_file(path: String) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        opener::open(path)?;
        Ok(())
    }
    #[cfg(target_os = "linux")]
    {
        Command::new("vim")
            .arg(path)
            .status()
            .expect("命令执行失败");
    }
    #[cfg(target_os = "macos")]
    {
        todo!("open macos file path");
    }
}

fn open_app(app_name: String) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        opener::open(app_name)?;
        Ok(())
    }
    #[cfg(target_os = "linux")]
    {
        Command::new(app_name)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        Ok(())
    }
    #[cfg(target_os = "macos")]
    {
        todo!("open macos file path");
    }
}
fn insert_into_template(template: &str, input: &str) -> String {
    if template.contains("{}") {
        return template.replace("{}", input);
    } else if input.is_empty() {
        return template.to_string();
    } else {
        return template.to_string() + input;
    }
}
