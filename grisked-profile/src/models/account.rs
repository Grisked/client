use serde::{Deserialize, Serialize};

use crate::models::bill::Bill;

use super::bill::BillType;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Account {
    pub name: String,
    #[serde(skip_serializing)]
    id: Option<usize>,
    default_balance: f64,
    bills: Vec<Bill>,
    pub color: [f32; 3],
}

impl Default for Account {
    fn default() -> Self {
        Self {
            name: "new_account".to_string(),
            id: None,
            default_balance: 0.0,
            bills: Vec::new(),
            color: [0.2, 0.2, 0.2],
        }
    }
}

impl Account {
    pub fn new(name: &str, default_balance: f64, bills: Vec<Bill>, color: [f32; 3]) -> Self {
        Self {
            name: name.to_string(),
            id: None,
            default_balance,
            bills,
            color,
        }
    }

    pub fn get_bills(&self) -> &Vec<Bill> {
        &self.bills
    }

    pub fn add_bill(&mut self, bill: Bill) {
        self.bills.push(bill);
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
            match bill.bill_type {
                BillType::Income => {
                    balance += bill.price;
                }
                BillType::Invoice => {
                    balance -= bill.price;
                }
            };
        }
        balance
    }

    pub fn pretty_account_balance(&self) -> String {
        let balance = self.get_account_balance();

        if balance > 0.0 {
            format!("+{:0.2} €", balance)
        } else {
            format!("{:0.2} €", balance)
        }
    }

    /// Returns the account balance if every bills were already dued.
    pub fn get_account_max_balance(&self) -> f32 {
        todo!()
    }

    /// Internal id
    pub fn get_account_id(&self) -> usize {
        self.id.unwrap()
    }

    pub fn register(&mut self, id: usize) {
        self.id = Some(id);
    }
}
