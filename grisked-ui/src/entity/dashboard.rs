use grisked_profile::profile::Profile;
use iced::{
    alignment, theme,
    widget::{button, column, container, row, text, Column, Container},
    Color, Length,
};

use crate::{
    entity::menu::MenuType,
    font::{FontFamily, FontType},
    stylesheet::{ButtonType, ContainerType},
    view::{View, ViewSize},
    Message,
};

pub fn dashboard(profile: &Profile, view: View) -> Container<Message> {
    let left_side = Column::new()
        .width(Length::FillPortion(1))
        .spacing(10)
        .push(recent_accounts(profile, view.clone()))
        .push(deadlines(profile, view.clone()));

    let right_side = Column::new()
        .width(Length::FillPortion(3))
        .spacing(10)
        .width(Length::FillPortion(2))
        .push(spendings(profile, view.clone()))
        .push(pins(profile, view.clone()));

    let container: Container<Message> = container(row!(left_side, right_side).spacing(20))
        .height(Length::FillPortion(7))
        .width(Length::Fill)
        .padding(50)
        .into();
    container
}

fn recent_accounts(profile: &Profile, view: View) -> Container<Message> {
    let container: Container<Message> = container(column!(
        button(
            FontType::Title
                .get_text("Comptes récents".to_string(), FontFamily::IndieFlower)
                .width(Length::Fill)
                .style(Color::from([0.2235, 0.0, 0.5294]))
                .size(ViewSize::Title.get_size(&view))
                .horizontal_alignment(alignment::Horizontal::Left)
        )
        .style(theme::Button::Custom(ButtonType::BoxIgnored.get_box()))
        .on_press(Message::MenuChanged(MenuType::Accounts)),
        {
            let mut column = Column::new().spacing(25);
            for account in &profile.accounts {
                column = column.push(column!(
                    row!(
                        text("• ")
                            .size(ViewSize::Text.get_size(&view))
                            .vertical_alignment(alignment::Vertical::Center)
                            .height(Length::Units(25)),
                        FontType::TextBold
                            .get_text(account.name.clone(), FontFamily::Kanit)
                            .size(ViewSize::Text.get_size(&view)),
                        FontType::TextBold
                            .get_text(
                                format!("{:0.2} €", &account.get_account_balance()),
                                FontFamily::Kanit
                            )
                            .size(ViewSize::Text.get_size(&view))
                            .horizontal_alignment(alignment::Horizontal::Right)
                            .width(Length::Fill)
                    ),
                    {
                        let mut c_bills = Column::new();
                        for bill in &account.bills {
                            c_bills = c_bills.push(
                                row!(
                                    text("• ")
                                        .size(ViewSize::Text.get_size(&view))
                                        .vertical_alignment(alignment::Vertical::Center)
                                        .height(Length::Units(20)),
                                    FontType::Text
                                        .get_text(bill.name.clone(), FontFamily::Kanit)
                                        .size(ViewSize::Text.get_size(&view)),
                                    {
                                        let mut text = FontType::Text
                                            .get_text(bill.pretty_price(), FontFamily::Kanit)
                                            .size(ViewSize::Text.get_size(&view));
                                        if bill.price < 0.0 {
                                            text =
                                                text.style(Color::from([(189.0 / 255.0), 0.0, 0.0]))
                                        } else {
                                            text =
                                                text.style(Color::from([0.0, (134.0 / 255.0), 0.0]))
                                        }
                                        text
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
        },
    ))
    .style(theme::Container::Custom(ContainerType::Box.get_box()))
    .padding(20);
    container
}

fn deadlines(_profile: &Profile, _view: View) -> Container<Message> {
    container(column!(
        text("Echéances en cours")
            .width(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Center),
        row!(text(" ")),
        row!(text("Prêt de la 4090 RTX"), text("834€")).spacing(20),
    ))
    .style(theme::Container::Custom(ContainerType::Box.get_box()))
    .padding(20)
}

fn spendings(_profile: &Profile, _view: View) -> Container<Message> {
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
    .padding(20)
}

fn pins(_profile: &Profile, _view: View) -> Container<Message> {
    container(column!(
        text("Rappels")
            .width(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Center),
        row!(text(" ")),
        row!(text("Payer la facture d'eau du 11/04/23"))
    ))
    .style(theme::Container::Custom(ContainerType::Box.get_box()))
    .padding(20)
}
