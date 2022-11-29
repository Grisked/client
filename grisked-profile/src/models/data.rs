use serde::{Deserialize, Serialize};

use crate::{
    models::{account::Account, label::Label},
    serde_utils::{load_json, save_json},
};

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(skip_serializing)]
    path: Option<String>,
    pub accounts: Vec<Account>,
    pub labels: Vec<Label>,
}

impl Default for Data {
    fn default() -> Self {
        match load_json::<Self>("data.json".to_string()) {
            Ok(mut data) => {
                data.path = Some("data.json".to_string());
                data
            }
            Err(err) => {
                println!("Error = {:?}", err);
                let data = Self {
                    path: Some("data.json".to_string()),
                    accounts: Vec::new(),
                    labels: Vec::new(),
                };
                data.save();
                data
            }
        }
    }
}

impl Data {
    pub fn save(&self) {
        save_json(self.path.as_ref().unwrap(), &self);
    }

    pub fn load(path: String) -> Result<Self, String> {
        match load_json::<Self>(path.clone()) {
            Ok(mut result) => {
                result.path = Some(path);
                Ok(result)
            }
            Err(err) => Err(err),
        }
    }
}
