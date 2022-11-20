use iced::widget::{button, column, container, row, Column};
use iced::{executor, theme};
use iced::{Application, Command, Element, Length, Settings, Theme};

use crate::entity::menu::*;
use crate::{Language, Message};

pub fn launch() -> iced::Result {
    Grisked::run(Settings {
        antialiasing: true,
        default_font: Some(include_bytes!("../../fonts/Inter-Regular.ttf")),
        ..Settings::default()
    })
}

#[derive(Default)]
struct Grisked {
    menu_type: MenuType,
    language: Language,
}

impl Application for Grisked {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Grisked::default(), Command::none())
    }

    fn title(&self) -> String {
        "Grisked".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::MenuChanged(menu_type) => self.menu_type = menu_type,
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let choose_menu = [
            MenuType::Home,
            MenuType::Accounts,
            MenuType::Charts,
            MenuType::Deadlines,
            MenuType::Backup,
        ]
        .iter()
        .fold(
            Column::new().spacing(10).height(Length::Fill),
            |column, option| {
                column.push(
                    button(row![option.get_icon(), option.get_name(&self.language)].spacing(10))
                        .on_press(crate::Message::MenuChanged(*option))
                        .padding(10)
                        .style(theme::Button::Primary),
                )
            },
        );

        let content = column![choose_menu];

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}
