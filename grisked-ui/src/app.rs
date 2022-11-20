use iced::alignment::{Horizontal, Vertical};
use iced::theme::Container;
use iced::widget::{self, button, column, container, row, text, Column};
use iced::{executor, theme, Alignment};
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
        let sidebar = [
            MenuType::Home,
            MenuType::Accounts,
            MenuType::Charts,
            MenuType::Deadlines,
            MenuType::Backup,
        ]
        .iter()
        .fold(
            Column::new()
                .spacing(10)
                .height(Length::Fill)
                .width(Length::FillPortion(1)),
            |column, option| {
                column.push(
                    button(row![option.get_icon(), option.get_name(&self.language)].spacing(10))
                        .on_press(crate::Message::MenuChanged(*option))
                        .padding(10)
                        .style(theme::Button::Primary)
                        .width(Length::Fill),
                )
            },
        );

        let sidebar_container: Element<Message> = container(sidebar)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .style(theme::Container::Box)
            .into();

        let context = match self.menu_type {
            MenuType::Home => {
                let value = Column::new()
                    .spacing(10)
                    .push(
                        container(column!(
                            text("Comptes récemments utilisés"),
                            row!(
                                column!(text("Compte 1"), text("Livret A"))
                                    .align_items(Alignment::Start),
                                column!(text("834€"), text("7493€")).align_items(Alignment::End)
                            )
                        ))
                        .style(Container::Box)
                        .padding(20),
                    )
                    .push(
                        container(column!(
                            text("Echéances en cours"),
                            row!(text("Prêt de la 4090 RTX"), text("10384€")),
                        ))
                        .style(Container::Box)
                        .padding(20),
                    )
                    .push(
                        container(column!(
                            text("Rappels"),
                            row!(text("Payer la facture d'eau du 11/04/23"))
                        ))
                        .style(Container::Box)
                        .padding(20),
                    );

                let container: Element<Message> = container(value)
                    .width(Length::FillPortion(3))
                    .height(Length::Fill)
                    .padding(50)
                    .into();

                Some(container)
            }
            _ => None,
        };

        match context {
            Some(context) => {
                let content = row![sidebar_container, context];
                container(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(20)
                    .into()
            }
            None => container(sidebar_container)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(20)
                .into(),
        }
    }
}
