use serde::{Deserialize, Serialize};

use crate::models::{account::Account, settings::Settings};

#[derive(Deserialize, Serialize, Default)]
pub struct Profile {
    profile_path: String,
    settings: Settings,
    pub accounts: Vec<Account>,
}

impl Profile {
    pub fn load_profile(_profile_path: &str) -> Option<Self> {
        todo!()
    }

    pub fn default_profile() -> Self {
        Self {
            profile_path: "profile.json".to_string(),
            settings: Settings::default(),
            accounts: Vec::new(),
        }
    }

    pub fn save_all(&self) {}

    pub fn save_settings(&self) {}
}
