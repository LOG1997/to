use crate::run::assort::{CommandType, match_command_type};
use anyhow::Result;
use shell_words;
use std::process::{Command, Stdio};

pub fn run_command(value: String, params: Vec<String>) -> Result<()> {
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
            open_web_page(value, params)?;
        }
        CommandType::DirectCommand => {
            opener::open(value)?;
        }
        CommandType::Other => {
            println!("so sorry,i cant run your command {}", value);
            let args = shell_words::split(value.as_str())?;
            if let Some(prog) = args.first() {
                Command::new(prog).args(&args[1..]).status()?;
            }
        }
    }
    Ok(())
}

fn open_web_page(url: String, parms: Vec<String>) -> Result<()> {
    // 不用区分环境了，webbrowser已经做了
    let result = insert_into_template(url.as_str(), parms.join(" ").as_str());
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
        opener::open(path)?;
        Ok(())
    }
    #[cfg(target_os = "macos")]
    {
        todo!("macos open browser of url");
    }
}

fn open_file(path: String) -> Result<()> {
    // 这个不用区分环境，opener已经做了
    opener::open(path)?;
    Ok(())
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
