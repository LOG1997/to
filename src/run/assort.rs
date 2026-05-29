use std::path::Path;
use which::which;

#[derive(Debug, PartialEq)]
pub enum CommandType {
    WebPage,
    LocalApp,
    DirPath,
    FilePath,
    DirectCommand,
    Other,
}

pub fn match_command_type(command: &str) -> CommandType {
    let command_trim = command.trim();
    if command_trim.starts_with("http") {
        return CommandType::WebPage;
    } else if which(command_trim).is_ok() {
        return CommandType::LocalApp;
    } else if Path::new(command_trim).is_dir() {
        return CommandType::DirPath;
    } else if Path::new(command_trim).is_file() {
        return CommandType::FilePath;
    }
    return CommandType::Other;
}

pub fn is_direct_command(command: &str) -> CommandType {
    let command_trim = command.trim();
    if command_trim.starts_with(".")
        || command_trim.starts_with("/")
        || command_trim.starts_with("http")
        || Path::new(command_trim).is_dir()
        || Path::new(command_trim).is_file()
    {
        return CommandType::DirectCommand;
    }
    return CommandType::Other;
}
