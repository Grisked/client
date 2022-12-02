use grisked_profile::{models::account::Account, profile::Profile};
use iced::widget::Container;

use crate::{entity, view::View, Language, Message};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum MenuType {
    #[default]
    Dashboard,
    Accounts,
    AccountData(Account),
    Deadlines,
    Charts,
    Backup,
}

impl MenuType {
    pub fn get_container(
        &self,
        profile: &Profile,
        _language: &Language,
        view: &View,
    ) -> Option<Container<'static, Message>> {
        match self {
            MenuType::Dashboard => Some(entity::dashboard(profile, view)),
            MenuType::Accounts => Some(entity::accounts(profile, view)),
            MenuType::Backup => Some(entity::backup(profile, view)),
            _ => None,
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
                _ => String::new(),
            },
            Language::FR => match self {
                Self::Dashboard => "DASHBOARD".to_string(),
                Self::Accounts => "COMPTES".to_string(),
                Self::Deadlines => "ECHÉANCES".to_string(),
                Self::Charts => "GRAPHIQUES".to_string(),
                Self::Backup => "SAUVEGARDE".to_string(),
                _ => String::new(),
            },
        }
    }
}
