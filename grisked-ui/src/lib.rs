pub mod app;
pub mod entity;
pub mod font;
pub mod stylesheets;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    MenuChanged(entity::menu::MenuType),
}

#[derive(Default, Clone)]
pub enum Language {
    #[default]
    EN,
    FR,
}
