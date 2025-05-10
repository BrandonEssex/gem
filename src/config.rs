use std::{collections::HashMap, fs, path::PathBuf};
use serde::{Deserialize, Serialize};

pub type KeyMap = HashMap<String, HashMap<String, String>>;

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
    let path = PathBuf::from("keymap.json");
    if let Ok(content) = fs::read_to_string(&path) {
        if let Ok(cfg) = serde_json::from_str::<KeyMap>(&content) {
            return cfg;
        }
    }
    default_keymap()
}

fn default_keymap() -> KeyMap {
    HashMap::from([("mindmap".into(), HashMap::from([("quit".into(), "ctrl+c".into())]))])
}
