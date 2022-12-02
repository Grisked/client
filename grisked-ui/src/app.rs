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
            Message::SaveRequested => {
                println!("Saving json files !");
                self.profile.save();
            }
            Message::AddAccount(account) => {
                self.profile.data.register_account(account);
            }
            Message::AddInvoice(account_id, bill) => {
                self.profile
                    .data
                    .get_account(account_id)
                    .unwrap()
                    .add_bill(bill);
            }
            Message::AddLabel(label) => {
                self.profile.data.add_label(label);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let navbar_container =
            navbar_container(&self.menu_type, self.language.clone(), self.view.clone());

        let context = self
            .menu_type
            .get_container(&self.profile, &self.language, &self.view);
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
