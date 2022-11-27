use serde::{Deserialize, Serialize};

use crate::models::bill::Bill;

#[derive(Deserialize, Serialize)]
pub struct Account {
    account_name: String,
    account_balance: f32,
    bills: Vec<Bill>,
}

impl Default for Account {
    fn default() -> Self {
        Self {
            account_name: "new_account".to_string(),
            account_balance: 0.0,
            bills: Vec::new(),
        }
    }
}

impl Account {
    pub fn remove_bill<P>(&mut self, filter: P)
    where
        P: FnMut(&mut Bill) -> bool,
    {
        self.bills.drain_filter(filter);
    }
}
