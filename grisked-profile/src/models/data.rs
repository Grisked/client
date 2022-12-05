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
    account_id: Option<usize>,
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

    pub fn get_label_by_id(&self, label_id: Option<usize>) -> Option<Label> {
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

    pub fn get_labels_rankings(&self, limit: Option<u32>) -> Vec<(Option<Label>, f64)> {
        let mut ranking: HashMap<Option<usize>, f64> = HashMap::new();

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

        match limit {
            Some(limit) => {
                let mut elements: Vec<(Option<Label>, f64)> = Vec::new();
                let mut total: f64 = 0.0;
                let mut i: u32 = 0;

                for element in vec {
                    if i < limit && element.0.is_some() {
                        i += 1;
                        elements.push(element);
                    } else {
                        total += element.1
                    }
                }
                elements.push((None, total));
                return elements;
            }
            None => vec,
        }
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
        let mut id = 0;
        match self.account_id {
            Some(result) => id = result,
            None => {}
        }
        self.account_id = Some(id + 1);
        let mut account = account;
        account.register(id);
        self.accounts.push(account);
    }

    pub fn get_account(&mut self, id: usize) -> Option<&mut Account> {
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

    pub fn add_label(&mut self, mut label: Label) {
        // We retrieve the highest label id
        let mut highest_id = 0;
        for label in &self.labels {
            if label.id > highest_id {
                highest_id = label.id;
            }
        }

        label.id = highest_id + 1;
        self.labels.push(label);
    }
}
