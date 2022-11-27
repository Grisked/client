use serde::{Deserialize, Serialize};

use crate::models::currency::Currency;

/// App settings data model
#[derive(Deserialize, Serialize)]
pub struct Settings {
    currencies: Vec<Currency>,
}

impl Default for Settings {
    fn default() -> Self {
        let usd = Currency::new('$', "United States dollar", "USD", 1.0);
        let eur = Currency::new('€', "Euro", "EUR", 0.97);
        let gpt = Currency::new('£', "British pound sterling", "GBP", 0.84);

        let currencies: Vec<Currency> = vec![usd, eur, gpt];

        Self { currencies }
    }
}

impl Settings {
    pub fn new(currencies: Vec<Currency>) -> Self {
        Self { currencies }
    }

    pub fn add_currency(&mut self, currency: Currency) {
        if !self.currencies.contains(&currency) {
            self.currencies.push(currency);
        }
    }

    /// TODO change .retain to drain_all
    pub fn remove_currency<P>(&mut self, filter: P)
    where
        P: FnMut(&mut Currency) -> bool,
    {
        self.currencies.drain_filter(filter);
    }
}
