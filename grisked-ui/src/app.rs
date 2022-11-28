use iced::alignment;
use iced::theme::Container;
use iced::widget::{button, column, container, row, text, Column};
use iced::{executor, theme};
use iced::{Application, Command, Element, Length, Settings, Theme};

use grisked_profile::profile::Profile;

use crate::entity::menu::*;
use crate::entity::recent_accounts::recent_accounts;
use crate::entity::sidebar::sidebar_container;
use crate::stylesheets::ContainerType;
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
    pub profile: Profile,
}

impl Application for Grisked {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let grisked = Grisked::default();

        (grisked, Command::none())
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
        let sidebar_container = sidebar_container(&self.menu_type, self.language.clone());

        let context = match self.menu_type {
            MenuType::Dashboard => {
                let left_side = Column::new()
                    .width(Length::FillPortion(2))
                    .spacing(10)
                    .push(recent_accounts(&self.profile))
                    .push(
                        container(column!(
                            text("Echéances en cours")
                                .width(Length::Fill)
                                .horizontal_alignment(alignment::Horizontal::Center),
                            row!(text(" ")),
                            row!(text("Prêt de la 4090 RTX"), text("834€")).spacing(20),
                        ))
                        .style(theme::Container::Custom(ContainerType::Box.get_box()))
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
                        .style(theme::Container::Custom(ContainerType::Box.get_box()))
                        .padding(20),
                    )
                    .width(Length::FillPortion(2))
                    .push(
                        container(column!(
                            text("Rappels")
                                .width(Length::Fill)
                                .horizontal_alignment(alignment::Horizontal::Center),
                            row!(text(" ")),
                            row!(text("Payer la facture d'eau du 11/04/23"))
                        ))
                        .style(theme::Container::Custom(ContainerType::Box.get_box()))
                        .padding(20),
                    );

                let container: Element<Message> =
                    container(row!(left_side, right_side).spacing(20))
                        .height(Length::FillPortion(6))
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
                        .style(theme::Container::Custom(ContainerType::Box.get_box()))
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
                        .style(theme::Container::Custom(ContainerType::Box.get_box()))
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
                    .height(Length::FillPortion(6))
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
                    .height(Length::FillPortion(6))
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
                    .style(theme::Container::Custom(
                        ContainerType::Background.get_box(),
                    ))
                    .into()
            }
            None => container(sidebar_container)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(20)
                .style(theme::Container::Custom(
                    ContainerType::Background.get_box(),
                ))
                .into(),
        }
    }
}
