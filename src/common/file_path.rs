use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

pub fn get_config_path() -> &'static PathBuf {
    static CONFIG_PATH: OnceLock<PathBuf> = OnceLock::new();
    let result_path = CONFIG_PATH.get_or_init(|| {
        let home = dirs::home_dir().expect("无法获取用户目录");
        home.join(".config").join("to").join("config.toml")
    });
    if !result_path.exists() {
        fs::create_dir_all(result_path.parent().unwrap()).unwrap();
    }
    result_path
}
