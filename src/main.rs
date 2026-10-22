use inquire::{Confirm, MultiSelect, Select, Text};
use portable_pty::{CommandBuilder, PtySize, native_pty_system};
use serde;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::io::Write;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Default)]
struct Config {
    shell: PathBuf,
    save_folder: PathBuf,
}

fn create_config(config_folder_path: &PathBuf) -> Config {
    let shell = Select::new("Shell?", vec!["bash", "zsh", "fish"])
        .prompt()
        .unwrap()
        .to_string();
    let save_folder = Text::new("Path to save folder?")
        .with_default("~/console-logger/")
        .prompt()
        .unwrap();

    let full_conf = Config {
        shell: PathBuf::from("/usr/bin/").join(shell),
        save_folder: PathBuf::from(save_folder.to_string()),
    };
    // write config to file
    match fs::create_dir_all(config_folder_path) {
        Ok(_) => (),
        Err(e) => panic!("An error occurred while creating the folders. {}", e),
    };
    match fs::write(
        config_folder_path.join("config.toml"),
        toml::to_string_pretty(&full_conf).unwrap(),
    ) {
        Ok(_) => (),
        Err(e) => panic!("An error occurred while creating the config file. {} ", e),
    }
    full_conf
}

fn main() {
    // TODO: Load Config
    let config_folder = dirs::config_dir().unwrap().join("console-logger");
    let config_file_content = fs::read_to_string(config_folder.join("config.toml"));
    let cfg: Config = match config_file_content {
        Ok(t) => toml::from_str(&t).unwrap(),
        Err(_) => create_config(&config_folder),
    };
    let pair = native_pty_system()
        .openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })
        .expect("PtyPair konnte nicht erstellt werden");
    let mut child = pair
        .slave
        .spawn_command(CommandBuilder::new(&cfg.shell))
        .expect("Shell konnte nicht gestartet werden");
    drop(pair.slave);
    let mut reader = pair.master.try_clone_reader().unwrap();
    let mut buf = [0u8; 4096];
    loop {
        let n = reader.read(&mut buf).unwrap_or(0);
        if n == 0 {
            break;
        } // 0 = Shell beendet
        print!("{}", String::from_utf8_lossy(&buf[..n]));
        std::io::stdout().flush().ok(); // braucht use std::io::Write
    }
}
