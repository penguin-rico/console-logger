
# 📝 console-logger

A small terminal session recorder written in Rust.
It runs your shell inside a pseudo-terminal and records the session, so you can hand in clean terminal output for university assignments.

---

## 📦 Installation

```bash
make                  # build as normal user
sudo make install     # install to /usr/local/bin
```

Without root:

```bash
make install PREFIX=~/.local
```

Alternative: `cargo install --path .`

Uninstall: `sudo make uninstall`

---

## 🚀 Usage

```bash
console-logger [COMMAND]
```

| Command        | Description                                  |
| -------------- | -------------------------------------------- |
| *(none)*       | Start the shell and record the session       |
| `config`       | Recreate the config interactively            |
| `ls`           | List the logs in the save folder             |
| `clear`        | Select and delete log files                  |
| `-h`, `--help` | Show help                                    |

Leave the recorded shell with `exit` (or `Ctrl+D`).

---

## ⚙️ Configuration

Created on first start at `~/.config/console-logger/config.toml`:

```toml
shell = "/bin/bash"
save_folder = "/home/user/logs"
```
