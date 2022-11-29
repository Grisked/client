use iced::{
    theme,
    widget::{button, container, row, Container, Row},
    Length,
};

use crate::{
    font::{icon, FontFamily, FontType},
    stylesheet::{ButtonType, ContainerType},
    view::{View, ViewSize},
    Language, Message,
};

use super::menu::MenuType;

pub fn navbar_container(
    menu_type: &MenuType,
    language: Language,
    view: View,
) -> Container<Message> {
    let mut sidebar = [
        MenuType::Dashboard,
        MenuType::Accounts,
        MenuType::Charts,
        MenuType::Deadlines,
        MenuType::Backup,
    ]
    .iter()
    .fold(
        Row::new()
            .spacing(10)
            .height(Length::FillPortion(1))
            .width(Length::Fill),
        |row, option| {
            let mut button = button(
                row![FontType::Header
                    .get_text(option.get_name(&language), FontFamily::Kanit)
                    .size(ViewSize::Header.get_size(&view))]
                .spacing(10),
            )
            .on_press(crate::Message::MenuChanged(*option))
            .padding(10);

            if menu_type == option {
                button = button.style(theme::Button::Custom(ButtonType::RegularSelected.get_box()));
            } else {
                button = button.style(theme::Button::Custom(ButtonType::RegularIgnored.get_box()));
            }

            row.push(button)
        },
    );

    sidebar = sidebar.push(icon('💰'));

    let sidebar_container: Container<Message> = container(sidebar)
        .width(Length::Fill)
        .height(Length::FillPortion(1))
        .padding(20)
        .style(theme::Container::Custom(ContainerType::Sidebar.get_box()));

    sidebar_container
}
