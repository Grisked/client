use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

use crate::models::{account::Account, settings::Settings};

#[derive(Serialize, Deserialize)]
pub struct Profile {
    #[serde(skip_serializing)]
    path: Option<String>,
    settings: Settings,
    pub accounts: Vec<Account>,
}

impl Default for Profile {
    fn default() -> Self {
        let profile = Self::load_profile("profile.json".to_string());

        match profile {
            Ok(mut profile) => {
                profile.path = Some("profile.json".to_string());
                profile
            }
            Err(err) => {
                println!("Error = {:?}", err);
                let profile = Self {
                    path: Some("profile.json".to_string()),
                    settings: Settings::default(),
                    accounts: Vec::new(),
                };
                profile.save();
                profile
            }
        }
    }
}

impl Profile {
    pub fn load_profile(profile_path: String) -> Result<Self, String> {
        let file = File::open(&profile_path);
        if file.is_err() {
            return Err(format!("{}", file.unwrap_err()));
        }
        let mut file = file.unwrap();
        let profile = serde_json::from_reader(&file);
        if profile.is_err() {
            let mut content = String::new();
            match file.read_to_string(&mut content) {
                Ok(_) => {
                    let _ = std::fs::write(format!("{}.old", &profile_path), content);
                }
                Err(_) => {}
            }
            return Err(format!("{:?}", profile.err().unwrap()));
        }
        let mut profile: Profile = profile.unwrap();
        profile.path = Some(profile_path);
        Ok(profile)
    }

    pub fn save(&self) {
        std::fs::write(
            &self.path.as_ref().unwrap(),
            serde_json::to_string_pretty(&self).unwrap(),
        )
        .unwrap();
    }
}
