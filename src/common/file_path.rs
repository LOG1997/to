use std::path::PathBuf;
use std::sync::OnceLock;

pub fn get_config_path() -> &'static PathBuf {
    static CONFIG_PATH: OnceLock<PathBuf> = OnceLock::new();
    CONFIG_PATH.get_or_init(|| {
        let home = dirs::home_dir().expect("无法获取用户目录");
        home.join(".config").join("to").join("config.toml")
    })
}
