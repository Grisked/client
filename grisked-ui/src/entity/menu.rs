use grisked_profile::profile::Profile;
use iced::widget::{Container, Text};

use crate::{entity, font::icon, view::View, Language, Message};

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
    pub fn get_container(
        self,
        profile: &Profile,
        _language: Language,
        view: View,
    ) -> Option<Container<Message>> {
        match self {
            MenuType::Dashboard => Some(entity::dashboard(profile, view)),
            MenuType::Accounts => Some(entity::accounts(profile, view)),
            MenuType::Backup => Some(entity::backup(profile, view)),
            _ => None,
        }
    }

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
                Self::Accounts => "ACCOUNTS".to_string(),
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
