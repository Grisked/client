pub mod app;
pub mod entity;
pub mod font;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    MenuChanged(entity::menu::MenuType),
}

#[derive(Default)]
pub enum Language {
    #[default]
    EN,
    FR,
}
