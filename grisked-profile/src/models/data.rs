use std::{cmp::Ordering, collections::HashMap};

use serde::{Deserialize, Serialize};

use crate::{
    models::{account::Account, label::Label},
    serde_utils::{load_json, save_json},
};

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(skip_serializing)]
    path: Option<String>,
    accounts: Vec<Account>,
    labels: Vec<Label>,
    #[serde(skip_serializing)]
    account_id: Option<u64>,
}

impl Default for Data {
    fn default() -> Self {
        match load_json::<Self>("data.json".to_string()) {
            Ok(mut data) => {
                data.path = Some("data.json".to_string());
                data.register_accounts();
                data
            }
            Err(err) => {
                println!("Error = {:?}", err);
                let data = Self {
                    path: Some("data.json".to_string()),
                    accounts: Vec::new(),
                    labels: Vec::new(),
                    account_id: Some(0),
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

    pub fn get_label_by_id(&self, label_id: Option<u16>) -> Option<Label> {
        match label_id {
            Some(label_id) => {
                for label in &self.labels {
                    if label.id == label_id {
                        return Some(label.clone());
                    }
                }
                None
            }
            None => None,
        }
    }

    pub fn get_labels_rankings(&self) -> Vec<(Option<Label>, f64)> {
        let mut ranking: HashMap<Option<u16>, f64> = HashMap::new();

        for account in &self.accounts {
            for bill in account.get_bills() {
                if bill.price >= 0.0 {
                    continue;
                }
                match ranking.get_key_value(&bill.label_id) {
                    Some((key, value)) => ranking.insert(*key, value + bill.price),
                    None => ranking.insert(bill.label_id, bill.price),
                };
            }
        }

        let mut vec: Vec<(Option<Label>, f64)> = Vec::from_iter(ranking)
            .iter()
            .map(|(label, price)| (self.get_label_by_id(*label), *price))
            .collect();
        vec.sort_by(|&(_, a), &(_, b)| {
            if a > b {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        vec
    }

    fn register_accounts(&mut self) {
        let mut id = self.account_id.unwrap();
        for account in &mut self.accounts {
            account.register(id);
            id += 1;
        }
        self.account_id = Some(id);
    }

    pub fn register_account(&mut self, account: Account) {
        let id = self.account_id.unwrap();
        self.account_id = Some(self.account_id.unwrap() + 1);
        let mut account = account;
        account.register(id);
    }

    pub fn get_account(&mut self, id: u64) -> Option<&mut Account> {
        for account in &mut self.accounts {
            if id == account.get_account_id() {
                return Some(account);
            }
        }
        None
    }

    pub fn get_accounts(&self) -> &Vec<Account> {
        &self.accounts
    }

    pub fn get_labels(&self) -> &Vec<Label> {
        &self.labels
    }

    pub fn add_label(&mut self, label: Label) {
        self.labels.push(label);
    }
}
