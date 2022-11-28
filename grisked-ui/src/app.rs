use iced::alignment;
use iced::theme::Container;
use iced::widget::{button, column, container, row, text, Column, Row};
use iced::{executor, theme};
use iced::{Application, Command, Element, Length, Settings, Theme, Color};

use grisked_profile::{
    models::{account::Account, bill::Bill},
    profile::Profile,
};

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
    pub profile: Profile,
}

impl Application for Grisked {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut grisked = Grisked::default();
        let bills = vec![
            Bill::new(1, "Clavier Gaming".to_string(), -140.0, 120),
            Bill::new(2, "Fiverr".to_string(), 220.0, 120),
        ];
        let account = Account::new("Compte Commun", 320.0, bills);

        grisked.profile.accounts.push(account);

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
                            .get_text("Comptes récents".to_string(), FontFamily::IndieFlower)
                                .width(Length::Fill)
                                .style(Color::from([0.2235, 0.0, 0.5294]))
                                .size(30)
                                .horizontal_alignment(alignment::Horizontal::Left),
                            {
                                let mut column = Column::new().spacing(25);
                                for account in &self.profile.accounts {
                                    column = column.push(column!(
                                        row!(
                                            text("• ").size(30).vertical_alignment(alignment::Vertical::Center).height(Length::Units(25)),
                                            FontType::Text
                                                .get_text(account.name.clone(), FontFamily::Kanit).size(25),
                                            FontType::Text
                                                .get_text(
                                                    format!("{:0.2} €", &account.get_account_balance()),
                                                    FontFamily::Kanit
                                                ).size(25)
                                            .horizontal_alignment(alignment::Horizontal::Right)
                                            .width(Length::Fill)
                                        ),
                                        {
                                            let mut c_bills = Column::new();
                                            for bill in &account.bills {
                                                c_bills = c_bills.push(
                                                    row!(
                                                        text("• ").size(30).vertical_alignment(alignment::Vertical::Center).height(Length::Units(20)),
                                                        FontType::Text.get_text(
                                                            bill.name.clone(),
                                                            FontFamily::Kanit
                                                        ),
                                                        if bill.price < 0.0 {
                                                            FontType::Text.get_text(
                                                                format!("{:0.2} €", &bill.price),
                                                                FontFamily::Kanit
                                                            )
                                                            .style(Color::from([0.5, 0.0, 0.0]))
                                                        } else {
                                                            FontType::Text.get_text(
                                                                format!("+{:0.2} €", &bill.price),
                                                                FontFamily::Kanit
                                                            )
                                                            .style(Color::from([0.0, 0.5, 0.0]))
                                                        }
                                                        .horizontal_alignment(alignment::Horizontal::Right)
                                                        .width(Length::Fill)
                                                    )
                                                    .padding([0, 0, 0, 20]),
                                                );
                                            }
                                            c_bills
                                        }
                                    ))
                                }
                                column
                            }
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
                        .height(Length::FillPortion(5))
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
