use crate::{
    models::{account::Account, settings::Settings},
    serde_utils::{load_json, save_json},
};

pub struct Profile {
    path: Option<String>,
    settings: Settings,
    pub accounts: Vec<Account>,
}

impl Default for Profile {
    fn default() -> Self {
        let profile = Self::load("profile.json".to_string(), "settings.json".to_string());

        match profile {
            Ok(profile) => profile,
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
    pub fn load(profile_path: String, settings_path: String) -> Result<Self, String> {
        match Settings::load(settings_path) {
            Ok(settings) => match load_json::<Vec<Account>>(profile_path.clone()) {
                Ok(accounts) => Ok(Self {
                    path: Some(profile_path),
                    settings,
                    accounts,
                }),
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }

    pub fn save(&self) {
        self.settings.save();
        self.save_accounts();
    }

    pub fn save_settings(&self) {
        self.settings.save();
    }

    pub fn save_accounts(&self) {
        save_json::<Vec<Account>>(self.path.as_ref().unwrap(), &self.accounts);
    }
}
