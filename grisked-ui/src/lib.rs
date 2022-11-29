use iced::keyboard::{KeyCode, Modifiers};

pub mod app;
pub mod entity;
pub mod font;
pub mod handler;
pub mod stylesheet;
pub mod view;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    MenuChanged(entity::menu::MenuType),
    KeyPressed(KeyCode, Modifiers),
}

#[derive(Default, Clone)]
pub enum Language {
    #[default]
    EN,
    FR,
}
