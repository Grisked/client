use iced::{widget, Theme};

pub mod label_square;
pub mod spendings_chart;

mod account_button;
mod background;
mod box_button;
mod custom_box;
mod navbar;
mod regular_button;
mod transparent_input;

pub enum TextInputType {
    Transparent,
}

impl TextInputType {
    pub fn get_box(self) -> Box<dyn widget::text_input::StyleSheet<Style = Theme> + 'static> {
        match self {
            Self::Transparent => Box::new(transparent_input::TransparentInput),
        }
    }
}

pub enum ButtonType {
    RegularSelected,
    RegularIgnored,
    BoxIgnored,
    AccountIgnored([f32; 3]),
    AccountEmpty,
}

impl ButtonType {
    pub fn get_box(self) -> Box<dyn widget::button::StyleSheet<Style = Theme> + 'static> {
        match self {
            Self::RegularSelected => Box::new(regular_button::SelectedButton),
            Self::BoxIgnored => Box::new(box_button::IgnoredButton),
            Self::RegularIgnored => Box::new(regular_button::IgnoredButton),
            Self::AccountIgnored(color) => Box::new(account_button::IgnoredButton(color)),
            Self::AccountEmpty => Box::new(account_button::EmptyButton),
        }
    }
}

pub enum ContainerType {
    Navbar,
    Background,
    Box,
}

impl ContainerType {
    pub fn get_box(&self) -> Box<dyn widget::container::StyleSheet<Style = Theme> + 'static> {
        match self {
            Self::Navbar => Box::new(navbar::Navbar),
            Self::Background => Box::new(background::AppBackground),
            Self::Box => Box::new(custom_box::CustomBox),
        }
    }
}
