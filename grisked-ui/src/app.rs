use grisked_profile::models::account::Account;
use grisked_profile::models::bill::{Bill, BillType};
use iced::widget::{column, container};
use iced::{executor, theme};
use iced::{subscription, Subscription};
use iced::{Application, Command, Element, Length, Settings, Theme};

use grisked_profile::profile::Profile;

use crate::entity::menu::*;
use crate::entity::navbar::navbar_container;
use crate::stylesheet::ContainerType;
use crate::view::View;
use crate::{handler, FieldSettings, Language, Message, UpdateBox};

pub fn launch() -> iced::Result {
    Grisked::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

#[derive(Default)]
pub struct Grisked {
    pub menu_type: MenuType,
    pub language: Language,
    pub view: View,
    pub profile: Profile,
    pub field_settings: FieldSettings,
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
                handler::handle_keys(keycode, modifiers, self);
            }
            Message::PreviousAccount => {
                if self.field_settings.account_id > 0 {
                    self.field_settings.account_id -= 1;
                }
            }
            Message::NextAccount => {
                if self.field_settings.account_id < self.profile.data.get_accounts().len() - 1 {
                    self.field_settings.account_id += 1;
                }
            }
            Message::SaveRequested => {
                println!("Saving json files !");
                self.profile.save();
            }
            Message::AddAccount => {
                // Check if every field is correct

                match self.field_settings.account_default_balance.parse::<f64>() {
                    Ok(default_balance) => {
                        self.profile.data.register_account(Account::new(
                            &self.field_settings.account_name,
                            default_balance,
                            Vec::new(),
                            [0.1, 0.5, 0.5],
                        ));
                        self.field_settings.account_name = String::new();
                        self.field_settings.account_default_balance = String::new();
                    }
                    _ => {}
                }
            }
            Message::AddLabel => {
                // Check if every field is corect
                if self.field_settings.label_name != "" {
                    self.field_settings.label_name = String::new();
                }
            }
            Message::AddInvoice => {
                // Check if every field is correct
                match self.field_settings.invoice_amount.parse::<f64>() {
                    Ok(invoice_amount) => {
                        match self
                            .profile
                            .data
                            .get_account(self.field_settings.account_id)
                        {
                            Some(account) => {
                                account.add_bill(Bill::new(
                                    BillType::Invoice,
                                    self.field_settings.invoice_name.clone(),
                                    invoice_amount,
                                    121,
                                    None,
                                ));
                                self.field_settings.invoice_name = String::new();
                                self.field_settings.invoice_amount = String::new();
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Message::AddIncome => {
                // Check if every field is correct
                self.field_settings.income_name = String::new();
                self.field_settings.invoice_amount = String::new();
            }
            Message::UpdateBox(value) => match value {
                UpdateBox::LabelName(name) => {
                    self.field_settings.label_name = name;
                }
                UpdateBox::AccountName(name) => {
                    self.field_settings.account_name = name;
                }
                UpdateBox::InvoiceName(name) => {
                    self.field_settings.invoice_name = name;
                }
                UpdateBox::IncomeName(name) => {
                    self.field_settings.income_name = name;
                }
                UpdateBox::InvoiceAmount(amount) => {
                    self.field_settings.invoice_amount = amount;
                }
                UpdateBox::IncomeAmount(amount) => {
                    self.field_settings.income_amount = amount;
                }
                UpdateBox::AccountDefaultBalance(balance) => {
                    self.field_settings.account_default_balance = balance;
                }
            },
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let navbar_container =
            navbar_container(&self.menu_type, self.language.clone(), self.view.clone());

        let context = self.menu_type.get_container(
            &self.profile,
            &self.language,
            &self.view,
            &self.field_settings,
        );
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
