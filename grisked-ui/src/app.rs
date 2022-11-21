use iced::theme::Container;
use iced::widget::{button, column, container, row, text, Column};
use iced::{executor, theme};
use iced::{Application, Command, Element, Length, Settings, Theme};
use iced::alignment;

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
                let mut button = button(row![option.get_icon(), option.get_name(&self.language)].spacing(10))
                .on_press(crate::Message::MenuChanged(*option))
                .padding(10)
                .width(Length::Fill);

                if &self.menu_type == option {
                    button = button.style(theme::Button::Primary)
                }
                else {
                    button = button.style(theme::Button::Secondary)
                }
                
                column.push(button)
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
                let left_side = Column::new()
                    .width(Length::FillPortion(2))
                    .spacing(10)
                    .push(
                        container(column!(
                            text("Comptes récemments utilisés").width(Length::Fill),
                            row!(text("Compte 1"), text("834€")).spacing(20),
                            row!(text("Livret A"), text("7493€")).spacing(25)
                        ))
                        .style(Container::Box)
                        .padding(20),
                    )
                    .push(
                        container(column!(
                            text("Echéances en cours").width(Length::Fill),
                            row!(text("Prêt de la 4090 RTX"), text("834€")).spacing(20),
                        ))
                        .style(Container::Box)
                        .padding(20),
                    )
                    .push(
                        container(column!(
                            text("Rappels").width(Length::Fill),
                            row!(text("Payer la facture d'eau du 11/04/23"))
                        ))
                        .style(Container::Box)
                        .padding(20),
                    );

                let right_side = Column::new()
                    .width(Length::FillPortion(2))
                    .spacing(10)
                    .push(
                        container(column!(
                            text("Dépenses du mois").width(Length::Fill),
                            text("[] Transports"),
                            text("[] Informatique"),
                            text("[] Sports"),
                            text("[] Alimentation"),
                        ))
                        .style(Container::Box)
                        .padding(20),
                    )
                    .width(Length::FillPortion(2));

                let container: Element<Message> = container(row!(left_side, right_side).spacing(20))
                    .width(Length::FillPortion(3))
                    .height(Length::Fill)
                    .padding(50)
                    .into();

                Some(container)
            }
            MenuType::Accounts => {
                let contents = Column::new()
                    .spacing(10)
                    .push(
                        container(column!(
                            text("Comptes courants").horizontal_alignment(alignment::Horizontal::Center).width(Length::Fill).size(30),
                            row!(text(" ")),
                            row!(text("Compte 1"), text("834€").horizontal_alignment(alignment::Horizontal::Right).width(Length::Fill)).spacing(25),
                            row!(text("Compte 2"), text("7493€").horizontal_alignment(alignment::Horizontal::Right).width(Length::Fill)).spacing(25),
                            row!(text(" ")),
                            row!(text("Total courants: 45678€").horizontal_alignment(alignment::Horizontal::Right).width(Length::Fill).size(15)),
                        ))
                        .style(Container::Box)
                        .padding(20),
                    )
                    .push(
                        container(column!(
                            text("Comptes épargnes").horizontal_alignment(alignment::Horizontal::Center).width(Length::Fill).size(30),
                            row!(text(" ")),
                            row!(text("Livret A"), text("10454€").horizontal_alignment(alignment::Horizontal::Right).width(Length::Fill)).spacing(25),
                            row!(text(" ")),
                            row!(text("Total épargnes: 45678€").horizontal_alignment(alignment::Horizontal::Right).width(Length::Fill).size(15)),
                        ))
                        .style(Container::Box)
                        .padding(20),
                    )
                    .push(
                        container(column!(
                            text("Total: 17483847€").horizontal_alignment(alignment::Horizontal::Right).width(Length::Fill),
                        ))
                        .style(Container::Box)
                        .padding([3, 20, 3, 20]),
                    );
                let container: Element<Message> = container(row!(contents))
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
