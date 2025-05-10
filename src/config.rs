use std::{collections::HashMap, fs, path::PathBuf};
use serde::{Deserialize, Serialize};

pub type KeyMap = HashMap<String, HashMap<String, String>>;

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
    HashMap::new()
}
