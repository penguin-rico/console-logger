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

fn create_config() -> Config {
    let default_config = Config {
        shell: "/bin/bash".to_string(),
        save_folder: dirs::home_dir().unwrap().join("logs"),
    };
    default_config
}

fn main() {
    // TODO: Load Config
    let config_folder = dirs::config_dir().unwrap().join("console-logger");
    let config_file_content = fs::read_to_string(config_folder.join("config.toml"));
    let cfg: Config = match config_file_content {
        Ok(t) => toml::from_str(&t).unwrap(),
        Err(e) => create_config(),
    };
    // TODO: Create config from user input
    //
}
