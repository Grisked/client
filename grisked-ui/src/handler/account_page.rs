use grisked_profile::models::{account::Account, data::Data};

use crate::app::Grisked;

pub enum AccountPage {
    PreviousAccount,
    NextAccount,
    AddAccount,
}

impl AccountPage {
    pub fn handle(self, app: &mut Grisked) {
        match self {
            Self::PreviousAccount => {
                let id = &mut app.field_settings.account_id;
                previous_account(id);
            }
            Self::NextAccount => {
                let id = &mut app.field_settings.account_id;
                let length = app.profile.data.get_accounts().len();
                next_account(id, length);
            }
            Self::AddAccount => {
                let name = &mut app.field_settings.account_name;
                let default_balance = &mut app.field_settings.account_default_balance;
                let data = &mut app.profile.data;
                add_account(name, default_balance, data);
            }
        }
    }
}
fn previous_account(id: &mut usize) {
    if *id > 0 {
        *id -= 1;
    }
}

fn next_account(id: &mut usize, length: usize) {
    if *id < length - 1 {
        *id += 1;
    }
}

fn add_account(name: &mut String, default_balance: &mut String, data: &mut Data) {
    match default_balance.parse::<f64>() {
        Ok(balance) => {
            data.register_account(Account::new(&name, balance, Vec::new(), [0.1, 0.5, 0.5]));
            *name = String::new();
            *default_balance = String::new();
        }
        _ => {}
    }
}
