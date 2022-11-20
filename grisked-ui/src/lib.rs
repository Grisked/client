pub mod app;
pub mod entity;

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
