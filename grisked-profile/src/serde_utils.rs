use std::{fs::File, io::Read};

use serde::{de::DeserializeOwned, Serialize};

pub fn load_json<T: DeserializeOwned>(file_path: String) -> Result<T, String> {
    let file = File::open(&file_path);
    if file.is_err() {
        return Err(format!("{}", file.unwrap_err()));
    }
    let mut file = file.unwrap();
    let settings = serde_json::from_reader(&file);
    if settings.is_err() {
        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Ok(_) => {
                let _ = std::fs::write(format!("{}.old", &file_path), content);
            }
            Err(_) => {}
        }
        return Err(format!("{:?}", settings.err().unwrap()));
    }
    Ok(settings.unwrap())
}

pub fn save_json<T: Serialize>(target_path: &str, target: &T) {
    std::fs::write(target_path, serde_json::to_string(target).unwrap()).unwrap();
}
