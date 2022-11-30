use iced::{
    alignment, theme,
    widget::{button, column, container, row, svg, text, Column, Container},
    Color, Length,
};

use grisked_profile::{
    models::{account::Account, bill::Bill},
    profile::Profile,
};

use crate::{
    entity::menu::MenuType,
    font::{FontFamily, FontType},
    stylesheet::{spendings_chart, ButtonType, ContainerType},
    view::{View, ViewSize},
    Message,
};

pub fn dashboard(profile: &Profile, view: &View) -> Container<'static, Message> {
    let left_side = Column::new()
        .width(Length::FillPortion(1))
        .spacing(10)
        .push(recent_accounts(profile, view))
        .push(deadlines(profile, view));

    let right_side = Column::new()
        .width(Length::FillPortion(3))
        .spacing(10)
        .width(Length::FillPortion(2))
        .push(spendings(profile, view))
        .push(pins(profile, view));

    let container: Container<Message> = container(row!(left_side, right_side).spacing(20))
        .height(Length::FillPortion(7))
        .width(Length::Fill)
        .padding(50)
        .into();
    container
}

fn get_bills(bills: &Vec<Bill>, view: &View) -> Column<'static, Message> {
    let mut c_bills = Column::new();
    for bill in bills {
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
                        text = text.style(Color::from([(189.0 / 255.0), 0.0, 0.0]))
                    } else {
                        text = text.style(Color::from([0.0, (134.0 / 255.0), 0.0]))
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

fn get_account(account: &Account, view: &View) -> Column<'static, Message> {
    column!(
        row!(
            text("• ")
                .size(ViewSize::Text.get_size(view))
                .vertical_alignment(alignment::Vertical::Center)
                .height(Length::Units(25)),
            FontType::TextBold
                .get_text(account.name.clone(), FontFamily::Kanit)
                .size(ViewSize::Text.get_size(view)),
            FontType::TextBold
                .get_text(
                    format!("{:0.2} €", &account.get_account_balance()),
                    FontFamily::Kanit
                )
                .size(ViewSize::Text.get_size(view))
                .horizontal_alignment(alignment::Horizontal::Right)
                .width(Length::Fill)
        ),
        get_bills(&account.bills, view)
    )
}

fn recent_accounts(profile: &Profile, view: &View) -> Container<'static, Message> {
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
            for account in &profile.data.accounts {
                column = column.push(get_account(account, view))
            }
            column
        },
    ))
    .style(theme::Container::Custom(ContainerType::Box.get_box()))
    .padding(20);
    container
}

fn deadlines(_profile: &Profile, _view: &View) -> Container<'static, Message> {
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

fn spendings(profile: &Profile, _view: &View) -> Container<'static, Message> {
    /*let rankings = profile.data.get_labels_ranking();
    let mut total = 0.0;
    for ranking in &rankings {
        total += ranking.1;
    }
    let mut percentages: Vec<(Option<Label>, f64)> = Vec::new();
    for ranking in rankings {
        percentages.push((ranking.0, ranking.1 / total));
    } Convert to percentage, but it works fine without it, weirdly.. */
    let rankings = profile.data.get_labels_rankings();
    let _ = spendings_chart::draw(&rankings);

    container(column!(
        text("Dépenses du mois")
            .width(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Center),
        container(svg(svg::Handle::from_path("assets/pie-chart.svg"))),
        {
            let mut column = Column::new();
            for ranking in rankings {
                match ranking.0 {
                    Some(ranking) => column = column.push(text(format!("[] {}", &ranking.name))),
                    None => column = column.push(text(format!("[] Autre"))),
                };
            }

            column
        }
    ))
    .style(theme::Container::Custom(ContainerType::Box.get_box()))
    .padding(20)
}

fn pins(_profile: &Profile, _view: &View) -> Container<'static, Message> {
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
