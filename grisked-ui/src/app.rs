use iced::widget::{column, container};
use iced::{executor, theme};
use iced::{subscription, Subscription};
use iced::{Application, Command, Element, Length, Settings, Theme};

use grisked_profile::profile::Profile;

use crate::entity::menu::*;
use crate::entity::navbar::navbar_container;
use crate::stylesheet::ContainerType;
use crate::view::View;
use crate::{handler, Language, Message};

pub fn launch() -> iced::Result {
    Grisked::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

#[derive(Default)]
struct Grisked {
    menu_type: MenuType,
    language: Language,
    view: View,
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
            Message::KeyPressed(keycode, modifiers) => {
                handler::zoom::handle(keycode, modifiers, &mut self.view)
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let navbar_container =
            navbar_container(&self.menu_type, self.language.clone(), self.view.clone());

        let context = self.menu_type.clone().get_container(
            &self.profile,
            self.language.clone(),
            self.view.clone(),
        );
        /*
                let context = match self.menu_type {
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
        */
        match context {
            Some(context) => {
                let content = column![navbar_container, context];
                container(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(theme::Container::Custom(
                        ContainerType::Background.get_box(),
                    ))
                    .into()
            }
            None => container(navbar_container)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(theme::Container::Custom(
                    ContainerType::Background.get_box(),
                ))
                .into(),
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        use iced::event::Event;
        use iced::keyboard;

        subscription::events_with(|event, _status| match event {
            Event::Keyboard(e) => match e {
                keyboard::Event::KeyPressed {
                    key_code,
                    modifiers,
                } => Some(Message::KeyPressed(key_code, modifiers)),
                _ => None,
            },
            _ => None,
        })
    }
}
