#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MenuType {
    #[default]
    Home,
    Accounts,
    Deadlines,
    Charts,
    Backup,
}

impl MenuType {
    pub fn get_icon(&self) -> &str {
        match self {
            Self::Home => "🏠",
            Self::Accounts => "€",
            Self::Deadlines => "📝",
            Self::Charts => "📈",
            Self::Backup => "💾",
        }
    }
}
