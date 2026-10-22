use chrono::Local;
use inquire::MultiSelect;
use inquire::{Confirm, Select, Text};
use portable_pty::{CommandBuilder, PtySize, native_pty_system};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Default, Debug)]
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
        .with_default(
            &dirs::home_dir()
                .unwrap()
                .join(".console-logger")
                .to_string_lossy(),
        )
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

fn print_centered_banner(path: &str, term_width: usize) {
    let border = "======================================================================";
    let line1 = "Console logger running successfully in the background.";
    let line2 = "Your session will be recorded and saved under:";
    let line3 = format!("> {}", path);

    // Packe alle Zeilen in ein Array, um sie in einer Schleife zu zentrieren
    let lines = [border, line1, line2, &line3, border];

    for line in lines {
        let text_len = line.chars().count();

        // Berechne die nötigen Leerzeichen auf der linken Seite
        let pad_left = if term_width > text_len {
            (term_width - text_len) / 2
        } else {
            0
        };

        // Drucke die Leerzeichen und direkt danach den Text
        println!("{}{}", " ".repeat(pad_left), line);
    }
}

fn main() {
    let config_folder = dirs::config_dir().unwrap().join("console-logger");
    let args: Vec<String> = std::env::args().collect();

    let config_file_content = fs::read_to_string(config_folder.join("config.toml"));

    let cfg: Config = match config_file_content {
        Ok(t) => toml::from_str(&t).unwrap(),
        Err(_) => create_config(&config_folder),
    };

    if args.len() > 1 {
        if args[1] == "config" {
            create_config(&config_folder);
            return;
        }
        if args[1] == "clear" {
            let mut namen: Vec<String> = fs::read_dir(&cfg.save_folder)
                .unwrap()
                .flatten()
                .filter(|e| e.path().is_file())
                .map(|e| e.file_name().to_string_lossy().to_string())
                .collect();
            namen.sort();

            if namen.is_empty() {
                println!("No files.");
                return;
            }
            const ALL: &str = "[ All files ]";
            let mut optionen = vec![ALL.to_string()];
            optionen.extend(namen.clone());

            let auswahl = MultiSelect::new("Which files should be deleted?", optionen)
                .prompt()
                .unwrap();

            let zu_loeschen = if auswahl.iter().any(|a| a == ALL) {
                namen // alle
            } else {
                auswahl // nur die gewählten
            };
            let ok = Confirm::new(&format!("Delete {} file(s)?", zu_loeschen.len()))
                .with_default(false)
                .prompt()
                .unwrap();
            if !ok {
                return;
            }
            for name in zu_loeschen {
                fs::remove_file(cfg.save_folder.join(&name)).unwrap();
            }
            return;
        }
        if args[1] == "ls" {
            let out = Command::new("ls")
                .arg("-a")
                .arg(&cfg.save_folder)
                .output()
                .unwrap();
            println!("{}", String::from_utf8_lossy(&out.stdout));
            return;
        }
        if args[1] == "-h" || args[1] == "--help" {
            println!("Usage: console-logger [COMMAND]");
            println!();
            println!("Commands:");
            println!("  (none)    Start shell and record the session");
            println!("  config    Recreate the config");
            println!("  ls        List logs in the save folder");
            println!("  -h        Show this help");
            return;
        }
        println!("Unsupported argumet: {}, -h help", args[1]);
        return;
    }

    fs::create_dir_all(&cfg.save_folder).unwrap();
    let ts = Local::now().format("%d.%m.%Y %H:%M:%S");

    let session_name = Text::new("Session Name?")
        .with_default(&ts.to_string())
        .prompt()
        .unwrap();
    print!("\x1B[2J\x1B[1;1H");

    let (cols, rows) = crossterm::terminal::size().unwrap();
    print_centered_banner(
        &cfg.save_folder.join(&session_name).to_string_lossy(),
        cols as usize,
    );
    let pair = native_pty_system()
        .openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })
        .expect("PtyPair konnte nicht erstellt werden");
    pair.slave
        .spawn_command(CommandBuilder::new(&cfg.shell))
        .expect("Shell konnte nicht gestartet werden");
    drop(pair.slave);
    let mut reader = pair.master.try_clone_reader().unwrap();
    let mut buf = [0u8; 4096];
    let mut writer = pair.master.take_writer().unwrap();

    // Create File
    let mut output_file = File::create(cfg.save_folder.join(session_name)).unwrap();
    writeln!(output_file, "Session started at: {ts}").unwrap();
    let shell_name = cfg.shell.into_os_string().into_string().unwrap();
    writeln!(output_file, "Shell: {shell_name}").unwrap();
    drop(shell_name);

    writeln!(output_file, "------------------------------------------").unwrap();
    writeln!(output_file, "```bash").unwrap();
    thread::spawn(move || {
        let mut stdin = std::io::stdin();
        let mut buf = [0u8; 1024];
        while let Ok(n) = stdin.read(&mut buf) {
            if n == 0 {
                break;
            }
            writer.write_all(&buf[..n]).ok();
        }
    });
    crossterm::terminal::enable_raw_mode().unwrap();
    let log = Arc::new(Mutex::new(&output_file));
    loop {
        let n = reader.read(&mut buf).unwrap_or(0);
        if n == 0 {
            break;
        } // 0 = Shell beendet
        log.lock().unwrap().write_all(&buf[..n]).unwrap();

        print!("{}", String::from_utf8_lossy(&buf[..n]));
        std::io::stdout().flush().ok(); // braucht use std::io::Write
    }
    writeln!(&output_file, "\n```").unwrap();

    writeln!(&output_file, "------------------------------------------").unwrap();
    writeln!(
        &output_file,
        "\nSession End: {}",
        Local::now().format("%d.%m.%Y %H:%M:%S")
    )
    .unwrap();
}
