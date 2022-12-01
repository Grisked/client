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
    pub accounts: Vec<Account>,
    pub labels: Vec<Label>,
}

impl Default for Data {
    fn default() -> Self {
        match load_json::<Self>("data.json".to_string()) {
            Ok(mut data) => {
                data.path = Some("data.json".to_string());
                data
            }
            Err(err) => {
                println!("Error = {:?}", err);
                let data = Self {
                    path: Some("data.json".to_string()),
                    accounts: Vec::new(),
                    labels: Vec::new(),
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
            for bill in &account.bills {
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
}
