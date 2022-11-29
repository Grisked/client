use serde::{Deserialize, Serialize};

use crate::{
    models::currency::Currency,
    serde_utils::{load_json, save_json},
};

/// App settings data model
#[derive(Deserialize, Serialize)]
pub struct Settings {
    #[serde(skip_serializing)]
    path: Option<String>,
    currencies: Vec<Currency>,
}

impl Default for Settings {
    fn default() -> Self {
        match load_json::<Self>("settings.json".to_string()) {
            Ok(mut settings) => {
                settings.path = Some("settings.json".to_string());
                settings
            }
            Err(err) => {
                println!("Error = {:?}", err);
                let usd = Currency::new('$', "United States dollar", "USD", 1.0);
                let eur = Currency::new('€', "Euro", "EUR", 0.97);
                let gpt = Currency::new('£', "British pound sterling", "GBP", 0.84);

                let settings = Self {
                    path: Some("settings.json".to_string()),
                    currencies: vec![usd, eur, gpt],
                };
                settings.save();
                settings
            }
        }
    }
}

impl Settings {
    pub fn add_currency(&mut self, currency: Currency) {
        if !self.currencies.contains(&currency) {
            self.currencies.push(currency);
        }
    }

    pub fn remove_currency<P>(&mut self, filter: P)
    where
        P: FnMut(&mut Currency) -> bool,
    {
        self.currencies.drain_filter(filter);
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

    pub fn save(&self) {
        save_json(self.path.as_ref().unwrap(), &self);
    }
}
