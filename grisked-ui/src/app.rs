use iced::alignment;
use iced::theme::Container;
use iced::widget::{button, column, container, row, text, Column, Row};
use iced::{executor, theme};
use iced::{Application, Command, Element, Length, Settings, Theme};

use crate::entity::menu::*;
use crate::font::*;
use crate::{Language, Message};

pub fn launch() -> iced::Result {
    Grisked::run(Settings {
        antialiasing: true,
        default_font: Some(include_bytes!("../../fonts/Kanit/200.ttf")),
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
                        .get_text(option.get_name(&self.language), FontFamily::Kanit)]
                    .spacing(10),
                )
                .on_press(crate::Message::MenuChanged(*option))
                .padding(10);

                if &self.menu_type == option {
                    button = button.style(theme::Button::Primary)
                } else {
                    button = button.style(theme::Button::Secondary)
                }

                row.push(button)
            },
        );

        sidebar = sidebar.push(icon('💰'));

        let sidebar_container: Element<Message> = container(sidebar)
            .width(Length::Fill)
            .height(Length::FillPortion(1))
            .padding(20)
            .style(theme::Container::Box)
            .into();

        let context = match self.menu_type {
            MenuType::Dashboard => {
                let left_side = Column::new()
                    .width(Length::FillPortion(2))
                    .spacing(10)
                    .push(
                        container(column!(
                            FontType::Title
                                .get_text("Comptes récents", FontFamily::IndieFlower)
                                .width(Length::Fill)
                                .horizontal_alignment(alignment::Horizontal::Left),
                            column!(
                                column!(
                                    row!(
                                        FontType::Text.get_text("Compte commun", FontFamily::Kanit),
                                        FontType::Text
                                            .get_text("400€", FontFamily::Kanit)
                                            .horizontal_alignment(alignment::Horizontal::Right)
                                    ),
                                    row!(text("Clavier gaming"), text("-140€"))
                                        .padding([0, 0, 0, 20]),
                                    row!(text("Fiverr"), text("+220€")).padding([0, 0, 0, 20])
                                ),
                                column!(
                                    row!(
                                        FontType::Text.get_text("Livret A", FontFamily::Kanit),
                                        FontType::Text
                                            .get_text("2520.90€", FontFamily::Kanit)
                                            .horizontal_alignment(alignment::Horizontal::Right)
                                    ),
                                    row!(text("Grand mère"), text("+20.70€"))
                                        .padding([0, 0, 0, 20]),
                                )
                            )
                            .spacing(25),
                        ))
                        .style(Container::Box)
                        .padding(20),
                    )
                    .push(
                        container(column!(
                            text("Echéances en cours")
                                .width(Length::Fill)
                                .horizontal_alignment(alignment::Horizontal::Center),
                            row!(text(" ")),
                            row!(text("Prêt de la 4090 RTX"), text("834€")).spacing(20),
                        ))
                        .style(Container::Box)
                        .padding(20),
                    )
                    .push(
                        container(column!(
                            text("Rappels")
                                .width(Length::Fill)
                                .horizontal_alignment(alignment::Horizontal::Center),
                            row!(text(" ")),
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
                            text("Dépenses du mois")
                                .width(Length::Fill)
                                .horizontal_alignment(alignment::Horizontal::Center),
                            row!(text(" ")),
                            text("[] Transports"),
                            text("[] Informatique"),
                            text("[] Sports"),
                            text("[] Alimentation"),
                        ))
                        .style(Container::Box)
                        .padding(20),
                    )
                    .width(Length::FillPortion(2));

                let container: Element<Message> =
                    container(row!(left_side, right_side).spacing(20))
                        .height(Length::FillPortion(3))
                        .width(Length::Fill)
                        .padding(50)
                        .into();

                Some(container)
            }
            MenuType::Accounts => {
                let contents = Column::new()
                    .spacing(10)
                    .push(
                        container(column!(
                            text("Comptes courants")
                                .horizontal_alignment(alignment::Horizontal::Center)
                                .width(Length::Fill)
                                .size(30),
                            row!(text(" ")),
                            row!(
                                text("Compte 1"),
                                text("834€")
                                    .horizontal_alignment(alignment::Horizontal::Right)
                                    .width(Length::Fill)
                            )
                            .spacing(25),
                            row!(
                                text("Compte 2"),
                                text("7493€")
                                    .horizontal_alignment(alignment::Horizontal::Right)
                                    .width(Length::Fill)
                            )
                            .spacing(25),
                            row!(text(" ")),
                            row!(text("Total courants: 45678€")
                                .horizontal_alignment(alignment::Horizontal::Right)
                                .width(Length::Fill)
                                .size(15)),
                        ))
                        .style(Container::Box)
                        .padding(20),
                    )
                    .push(
                        container(column!(
                            text("Comptes épargnes")
                                .horizontal_alignment(alignment::Horizontal::Center)
                                .width(Length::Fill)
                                .size(30),
                            row!(text(" ")),
                            row!(
                                text("Livret A"),
                                text("10454€")
                                    .horizontal_alignment(alignment::Horizontal::Right)
                                    .width(Length::Fill)
                            )
                            .spacing(25),
                            row!(text(" ")),
                            row!(text("Total épargnes: 45678€")
                                .horizontal_alignment(alignment::Horizontal::Right)
                                .width(Length::Fill)
                                .size(15)),
                        ))
                        .style(Container::Box)
                        .padding(20),
                    )
                    .push(
                        container(column!(text("Total: 17483847€")
                            .horizontal_alignment(alignment::Horizontal::Right)
                            .width(Length::Fill),))
                        .style(Container::Box)
                        .padding([3, 20, 3, 20]),
                    );
                let container: Element<Message> = container(row!(contents))
                    .height(Length::FillPortion(3))
                    .width(Length::Fill)
                    .padding(50)
                    .into();

                Some(container)
            }
            MenuType::Backup => {
                let contents = Column::new().spacing(10).push(
                    container(column!(
                        button(
                            text("Sauvegarder")
                                .horizontal_alignment(alignment::Horizontal::Center)
                                .width(Length::Fill)
                                .size(30)
                        )
                        .width(Length::Fill),
                        row!(" "),
                        button(
                            text("Savegarder sous")
                                .horizontal_alignment(alignment::Horizontal::Center)
                                .width(Length::Fill)
                                .size(30)
                        )
                        .width(Length::Fill),
                        row!(" "),
                        button(
                            text("Charger")
                                .horizontal_alignment(alignment::Horizontal::Center)
                                .width(Length::Fill)
                                .size(30)
                        )
                        .width(Length::Fill),
                        row!(" "),
                        button(
                            text("Ouvrir le dossier de sauvegarde")
                                .horizontal_alignment(alignment::Horizontal::Center)
                                .width(Length::Fill)
                                .size(30)
                        )
                        .width(Length::Fill),
                    ))
                    .padding(20),
                );
                let container: Element<Message> = container(row!(contents))
                    .height(Length::FillPortion(3))
                    .width(Length::Fill)
                    .padding(50)
                    .into();

                Some(container)
            }
            _ => None,
        };

        match context {
            Some(context) => {
                let content = column![sidebar_container, context];
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
