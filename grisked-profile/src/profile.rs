use std::fs::File;

use serde::{Deserialize, Serialize};

use crate::models::{account::Account, settings::Settings};

#[derive(Serialize, Deserialize)]
pub struct Profile {
    #[serde(skip_serializing)]
    path: String,
    settings: Settings,
    pub accounts: Vec<Account>,
}

impl Default for Profile {
    fn default() -> Self {
        let profile = Self::load_profile("profile.json".to_string());

        match profile {
            Ok(profile) => profile,
            Err(err) => {
                println!("Error = {:?}", err);
                let profile = Self {
                    path: "profile.json".to_string(),
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
        let file = file.unwrap();
        let profile = serde_json::from_reader(file);
        if profile.is_err() {
            return Err(format!("{:?}", profile.err().unwrap()));
        }
        let mut profile: Profile = profile.unwrap();
        profile.path = profile_path;
        Ok(profile)
    }

    pub fn save(&self) {
        std::fs::write(&self.path, serde_json::to_string_pretty(&self).unwrap()).unwrap();
    }
}
