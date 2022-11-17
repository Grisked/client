use iced::widget::{column, container, radio, Column};
use iced::{executor, Color};
use iced::{Application, Command, Element, Length, Settings, Theme};

use crate::entity::menu::*;
use crate::Message;

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
        .fold(Column::new().spacing(10), |column, option| {
            column.push(radio(
                format!("{} {:?}", option.get_icon(), option),
                *option,
                Some(*option),
                Message::MenuChanged,
            ))
        });

        let content = column![choose_menu];

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}
