use std::{collections::HashMap, fs, path::PathBuf};
use serde::{Deserialize, Serialize};

pub type KeyMap = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub keymap: KeyMap,
}

pub fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("gemx")
        .join("settings.json")
}

pub fn load_keymap() -> KeyMap {
    let path = config_path();
    if let Ok(content) = fs::read_to_string(&path) {
        if let Ok(cfg) = serde_json::from_str::<Config>(&content) {
            return cfg.keymap;
        }
    }
    default_keymap()
}

fn default_keymap() -> KeyMap {
    HashMap::from([
        ("quit".into(), "ctrl+q".into()),
        ("toggle_help".into(), "ctrl+h".into()),
        ("start_pomodoro".into(), "ctrl+p".into()),
        ("stop_pomodoro".into(), "ctrl+s".into()),
        ("toggle_timer".into(), "ctrl+t".into()),
        ("dashboard".into(), "ctrl+d".into()),
    ])
}
