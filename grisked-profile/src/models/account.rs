use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Account {
    account_name: String,
    account_balance: f32,
}
