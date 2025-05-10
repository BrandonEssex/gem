use std::fs;
use std::path::PathBuf;
use crate::storage::Storage;

pub fn process_incoming_folder(_storage: &mut Storage) -> std::io::Result<()> {
    let incoming = PathBuf::from("incoming");
    if !incoming.exists() {
        fs::create_dir_all(&incoming)?;
    }

    for entry in fs::read_dir(&incoming)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            // Future: parse and convert into Note or Todo
            println!("Ingested: {:?}", path.file_name());
        }
    }

    Ok(())
}
