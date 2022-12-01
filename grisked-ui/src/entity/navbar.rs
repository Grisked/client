use iced::{
    theme,
    widget::{button, container, row, svg, Container, Row},
    Length,
};

use crate::{
    font::{FontFamily, FontType},
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
    let navbar = [
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
            .width(Length::Fill)
            .push(
                svg(svg::Handle::from_path("assets/piggy-bank-fill.svg"))
                    .width(Length::Units(ViewSize::Header.get_size(&view)))
                    .height(Length::Units(ViewSize::Header.get_size(&view))),
            ),
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

    let question = svg(svg::Handle::from_path("assets/question-circle-fill.svg"))
        .width(Length::Units(ViewSize::Header.get_size(&view)))
        .height(Length::Units(ViewSize::Header.get_size(&view)));

    container(row!(row!(
        navbar.align_items(iced::Alignment::Center),
        container(question)
    )
    .align_items(iced::Alignment::Center)))
    .width(Length::Fill)
    .height(Length::FillPortion(1))
    .padding([20, 80, 20, 80])
    .style(theme::Container::Custom(ContainerType::Navbar.get_box()))
}
