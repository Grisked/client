use entity::menu::MenuType;
use grisked_profile::models::{account::Account, bill::Bill, label::Label};
use iced::keyboard::{KeyCode, Modifiers};

pub mod app;
pub mod entity;
pub mod font;
pub mod handler;
pub mod stylesheet;
pub mod view;

#[derive(Debug, Clone)]
pub enum Message {
    MenuChanged(MenuType),
    KeyPressed(KeyCode, Modifiers),
    SaveRequested,
    AddAccount(Account),
    AddLabel(Label),
    // account_id: u64
    AddInvoice(u64, Bill),
}

#[derive(Default, Clone)]
pub enum Language {
    #[default]
    EN,
    FR,
}
