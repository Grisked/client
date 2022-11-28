use iced::{widget, Theme};

mod background;
mod custom_box;
mod regular_button;
mod sidebar;

pub enum ButtonType {
    RegularSelected,
    RegularIgnored,
}

impl ButtonType {
    pub fn get_box(&self) -> Box<dyn widget::button::StyleSheet<Style = Theme> + 'static> {
        match self {
            Self::RegularSelected => Box::new(regular_button::SelectedButton),
            Self::RegularIgnored => Box::new(regular_button::IgnoredButton),
        }
    }
}

pub enum ContainerType {
    Sidebar,
    Background,
    Box,
}

impl ContainerType {
    pub fn get_box(&self) -> Box<dyn widget::container::StyleSheet<Style = Theme> + 'static> {
        match self {
            Self::Sidebar => Box::new(sidebar::Sidebar),
            Self::Background => Box::new(background::AppBackground),
            Self::Box => Box::new(custom_box::CustomBox),
        }
    }
}
