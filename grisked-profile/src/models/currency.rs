use serde::{Deserialize, Serialize};

/// Currency data model
#[derive(PartialEq, Deserialize, Serialize)]
pub struct Currency {
    pub symbol: char,
    pub name: String,
    pub alias: String,
    pub convert_rate: f32,
}

impl Currency {
    /// Create a new currency
    /// ```
    /// let symbol: char = '$'
    /// let name: &str = "United States dollar"
    /// let alias: &str = "USD"
    /// let convert_rate: f32 = 1.0
    /// let currency = Currency::new(symbol, name, alias, convert_rate);
    /// ```
    pub fn new(symbol: char, name: &str, alias: &str, convert_rate: f32) -> Self {
        Self {
            symbol,
            name: name.into(),
            alias: alias.into(),
            convert_rate,
        }
    }
}
