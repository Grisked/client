use crate::models::currency::Currency;

/// App settings data model
pub struct Settings {
    currencies: Vec<Currency>,
}

impl Settings {
    fn add_currency(&mut self, currency: Currency) {
        if !self.currencies.contains(&currency) {
            self.currencies.push(currency);
        }
    }

    pub fn remove_currency<P>(&mut self, filter: P)
    where
        P: FnMut(&Currency) -> bool,
    {
        self.currencies.retain(filter);
    }
}
