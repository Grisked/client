use serde::{Deserialize, Serialize};

use crate::models::bill::Bill;

#[derive(Deserialize, Serialize)]
pub struct Account {
    pub name: String,
    default_balance: f64,
    pub bills: Vec<Bill>,
}

impl Default for Account {
    fn default() -> Self {
        Self {
            name: "new_account".to_string(),
            default_balance: 0.0,
            bills: Vec::new(),
        }
    }
}

impl Account {
    pub fn new(name: &str, default_balance: f64, bills: Vec<Bill>) -> Self {
        Self {
            name: name.to_string(),
            default_balance,
            bills,
        }
    }

    pub fn remove_bill<P>(&mut self, filter: P)
    where
        P: FnMut(&mut Bill) -> bool,
    {
        self.bills.drain_filter(filter);
    }

    pub fn get_account_balance(&self) -> f64 {
        let mut balance: f64 = self.default_balance;

        for bill in &self.bills {
            balance += bill.price;
        }
        balance
    }

    /// Returns the account balance if every bills were already dued.
    pub fn get_account_max_balance(&self) -> f32 {
        todo!()
    }
}
