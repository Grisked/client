/// Currency data model
/// example:
/// symbol: char = '$'
/// name: String = "United States dollar"
/// alias: String = "USD"
/// convert_rate: f32 = 1.0f // The convert rate of $1 USD is $1 USD
#[derive(PartialEq)]
pub struct Currency {
    pub symbol: char,
    pub name: String,
    pub alias: String,
    pub convert_rate: f32,
}

impl Currency {
    pub fn new(symbol: char, name: &str, alias: &str, convert_rate: f32) -> Self {
        Self {
            symbol,
            name: name.into(),
            alias: alias.into(),
            convert_rate,
        }
    }
}
