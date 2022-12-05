use entity::menu::MenuType;

use iced::keyboard::{KeyCode, Modifiers};

pub mod app;
pub mod entity;
pub mod font;
pub mod handler;
pub mod stylesheet;
pub mod view;

#[derive(Default)]
pub struct FieldSettings {
    pub label_name: String,
    pub account_name: String,
    pub account_default_balance: String,
    pub invoice_name: String,
    pub income_name: String,
    pub invoice_amount: String,
    pub income_amount: String,
    pub account_id: usize,
}

#[derive(Debug, Clone)]
pub enum UpdateBox {
    LabelName(String),
    AccountName(String),
    InvoiceName(String),
    IncomeName(String),
    InvoiceAmount(String),
    IncomeAmount(String),
    AccountDefaultBalance(String),
}

#[derive(Debug, Clone)]
pub enum Message {
    MenuChanged(MenuType),
    KeyPressed(KeyCode, Modifiers),
    PreviousAccount,
    NextAccount,
    SaveRequested,
    AddAccount,
    AddLabel,
    AddInvoice,
    AddIncome,
    UpdateBox(UpdateBox),
}

#[derive(Default, Clone)]
pub enum Language {
    #[default]
    EN,
    FR,
}
