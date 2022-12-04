use entity::menu::MenuType;
use grisked_profile::models::{account::Account, bill::Bill, label::Label};
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
}

#[derive(Debug, Clone)]
pub enum UpdateBox {
    LabelName(String),
}

#[derive(Debug, Clone)]
pub enum Message {
    MenuChanged(MenuType),
    KeyPressed(KeyCode, Modifiers),
    SaveRequested,
    AddAccount(Account),
    AddLabel(Label),
    // usize: account_id
    AddInvoice(usize, Bill),
    UpdateBox(UpdateBox),
}

#[derive(Default, Clone)]
pub enum Language {
    #[default]
    EN,
    FR,
}
