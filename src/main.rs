use inquire::{Confirm, MultiSelect, Select};
use serde;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Default)]
struct Config {
    shell: String,
    save_folder: PathBuf,
}

fn create_config() -> String {
    let default_config = Config {
        shell: "/bin/bash".to_string(),
        save_folder: dirs::home_dir().unwrap().join("logs"),
    };
    toml::to_string(&default_config)?
}

fn main() {
    // TODO: Load Config
    let config_folder = dirs::config_dir().unwrap().join("console-logger");
    let config_file_content = fs::read_to_string(config_folder.join("config.toml"));
    let config_text = match config_file_content {
        Ok(t) => t,
        Err(e) => create_config(),
    };
    let cfg: config = toml::from_str(&config_file_content);
    // TODO: Create config from user input
    //
}
