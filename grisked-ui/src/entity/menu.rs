use iced::{
    alignment,
    widget::{text, Text},
    Font, Length,
};

use crate::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MenuType {
    #[default]
    Dashboard,
    Accounts,
    Deadlines,
    Charts,
    Backup,
}

impl MenuType {
    pub fn get_icon(&self) -> Text<'static> {
        match self {
            Self::Dashboard => icon('🏠'),
            Self::Accounts => icon('💰'),
            Self::Deadlines => icon('📝'),
            Self::Charts => icon('📈'),
            Self::Backup => icon('💾'),
        }
    }

    pub fn get_name(&self, language: &Language) -> String {
        match language {
            Language::EN => match self {
                Self::Dashboard => "DASHBOARD".to_string(),
                Self::Accounts => "ACCOUNT".to_string(),
                Self::Deadlines => "DEADLINES".to_string(),
                Self::Charts => "CHARTS".to_string(),
                Self::Backup => "BACKUP".to_string(),
            },
            Language::FR => match self {
                Self::Dashboard => "DASHBOARD".to_string(),
                Self::Accounts => "COMPTES".to_string(),
                Self::Deadlines => "ECHÉANCES".to_string(),
                Self::Charts => "GRAPHIQUES".to_string(),
                Self::Backup => "SAUVEGARDE".to_string(),
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
