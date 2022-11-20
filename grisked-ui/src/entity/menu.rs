use iced::{
    alignment,
    widget::{text, Text},
    Font, Length,
};

use crate::Language;

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
    pub fn get_icon(&self) -> Text<'static> {
        match self {
            Self::Home => icon('🏠'),
            Self::Accounts => icon('💰'),
            Self::Deadlines => icon('📝'),
            Self::Charts => icon('📈'),
            Self::Backup => icon('💾'),
        }
    }

    pub fn get_name(&self, language: &Language) -> &str {
        match language {
            Language::EN => match self {
                Self::Home => "Home",
                Self::Accounts => "Accounts",
                Self::Deadlines => "Deadlines",
                Self::Charts => "Charts",
                Self::Backup => "Backup",
            },
            Language::FR => match self {
                Self::Home => "Accueil",
                Self::Accounts => "Comptes",
                Self::Deadlines => "Echéances",
                Self::Charts => "Graphiques",
                Self::Backup => "Sauvegarde",
            },
        }
    }
}

// Fonts
const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../../../fonts/NotoEmoji-Regular.ttf"),
};

pub fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(ICONS)
        .width(Length::Units(20))
        .horizontal_alignment(alignment::Horizontal::Center)
        .size(20)
}
