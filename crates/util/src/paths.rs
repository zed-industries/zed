use std::path::PathBuf;

lazy_static::lazy_static! {
    pub static ref HOME: PathBuf = dirs::home_dir().expect("failed to determine home directory");
    pub static ref CONFIG_DIR: PathBuf = HOME.join(".config").join("zed");
    pub static ref LOGS_DIR: PathBuf = HOME.join("Library/Logs/Zed");
    pub static ref SUPPORT_DIR: PathBuf = HOME.join("Library/Application Support/Zed");
    pub static ref LANGUAGES_DIR: PathBuf = HOME.join("Library/Application Support/Zed/languages");
    pub static ref COPILOT_DIR: PathBuf = HOME.join("Library/Application Support/Zed/copilot");
    pub static ref DB_DIR: PathBuf = HOME.join("Library/Application Support/Zed/db");
    pub static ref SETTINGS: PathBuf = CONFIG_DIR.join("settings.json");
    pub static ref KEYMAP: PathBuf = CONFIG_DIR.join("keymap.json");
    pub static ref LAST_USERNAME: PathBuf = CONFIG_DIR.join("last-username.txt");
    pub static ref LOG: PathBuf = LOGS_DIR.join("Zed.log");
    pub static ref OLD_LOG: PathBuf = LOGS_DIR.join("Zed.log.old");
}

pub mod legacy {
    use std::path::PathBuf;

    lazy_static::lazy_static! {
        static ref CONFIG_DIR: PathBuf = super::HOME.join(".zed");
        pub static ref SETTINGS: PathBuf = CONFIG_DIR.join("settings.json");
        pub static ref KEYMAP: PathBuf = CONFIG_DIR.join("keymap.json");
    }
}
