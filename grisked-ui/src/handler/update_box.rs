use crate::{app::Grisked, UpdateBox};

pub fn handle(value: UpdateBox, app: &mut Grisked) {
    match value {
        UpdateBox::LabelName(name) => {
            app.field_settings.label_name = name;
        }
        UpdateBox::AccountName(name) => {
            app.field_settings.account_name = name;
        }
        UpdateBox::InvoiceName(name) => {
            app.field_settings.invoice_name = name;
        }
        UpdateBox::IncomeName(name) => {
            app.field_settings.income_name = name;
        }
        UpdateBox::InvoiceAmount(amount) => {
            app.field_settings.invoice_amount = amount;
        }
        UpdateBox::IncomeAmount(amount) => {
            app.field_settings.income_amount = amount;
        }
        UpdateBox::AccountDefaultBalance(balance) => {
            app.field_settings.account_default_balance = balance;
        }
    }
}
