use crate::models::{data::Data, settings::Settings};

pub struct Profile {
    settings: Settings,
    pub data: Data,
}

impl Default for Profile {
    fn default() -> Self {
        let profile = Self::load("data.json".to_string(), "settings.json".to_string());

        match profile {
            Ok(profile) => profile,
            Err(err) => {
                println!("Error = {:?}", err);
                let profile = Self {
                    settings: Settings::default(),
                    data: Data::default(),
                };
                profile.save();
                profile
            }
        }
    }
}

impl Profile {
    pub fn load(data_path: String, settings_path: String) -> Result<Self, String> {
        match Settings::load(settings_path) {
            Ok(settings) => match Data::load(data_path) {
                Ok(data) => Ok(Self { settings, data }),
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }

    pub fn save(&self) {
        self.save_settings();
        self.save_data();
    }

    pub fn save_settings(&self) {
        self.settings.save();
    }

    pub fn save_data(&self) {
        self.data.save();
    }
}
