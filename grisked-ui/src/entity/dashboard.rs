use iced::{
    alignment,
    alignment::Alignment,
    theme,
    widget::{button, column, container, row, svg, text, Canvas, Column, Container},
    Color, Length,
};

use grisked_profile::{
    models::{account::Account, bill::Bill},
    profile::Profile,
};

use crate::{
    entity::menu::MenuType,
    font::{FontFamily, FontType},
    stylesheet::{label_square::LabelSquare, spendings_chart, ButtonType, ContainerType},
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
        .padding(50);
    container
}

fn get_bills(bills: &Vec<Bill>, view: &View) -> Column<'static, Message> {
    let mut c_bills = Column::new();
    for (i, bill) in bills.iter().enumerate() {
        if i >= 3 {
            break;
        }
        c_bills = c_bills.push(
            row!(
                text("• ").size(ViewSize::Text.get_size(view)),
                FontType::Text
                    .get_text(bill.name.clone(), FontFamily::Kanit)
                    .size(ViewSize::Text.get_size(view)),
                {
                    let mut text = FontType::Text
                        .get_text(bill.pretty_price(), FontFamily::Kanit)
                        .size(ViewSize::Text.get_size(view));
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
            text("• ").size(ViewSize::Text.get_size(view)),
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
        get_bills(account.get_bills(), view)
    )
}

fn recent_accounts(profile: &Profile, view: &View) -> Container<'static, Message> {
    let container: Container<Message> = container(column!(
        button(
            FontType::Title
                .get_text("Comptes récents".to_string(), FontFamily::IndieFlower)
                .width(Length::Fill)
                .style(Color::from([0.2235, 0.0, 0.5294]))
                .size(ViewSize::Title.get_size(view))
                .horizontal_alignment(alignment::Horizontal::Left)
        )
        .style(theme::Button::Custom(ButtonType::BoxIgnored.get_box()))
        .on_press(Message::MenuChanged(MenuType::Accounts)),
        {
            let mut column = Column::new().spacing(25);
            for (i, element) in profile.data.get_accounts().iter().enumerate() {
                if i >= 2 {
                    break;
                }
                column = column.push(get_account(element, view))
            }
            column
        },
    ))
    .style(theme::Container::Custom(ContainerType::Box.get_box()))
    .padding(20);
    container
}

fn deadlines(_profile: &Profile, view: &View) -> Container<'static, Message> {
    container(column!(
        button(
            FontType::Title
                .get_text("Echéances en cours".to_string(), FontFamily::IndieFlower)
                .width(Length::Fill)
                .style(Color::from([0.2235, 0.0, 0.5294]))
                .size(ViewSize::Title.get_size(view))
                .horizontal_alignment(alignment::Horizontal::Left)
        )
        .style(theme::Button::Custom(ButtonType::BoxIgnored.get_box()))
        .on_press(Message::MenuChanged(MenuType::Deadlines)),
        {
            let mut column = Column::new();
            column = column.push(row!(
                text("• ")
                    .size(ViewSize::Text.get_size(view))
                    .vertical_alignment(alignment::Vertical::Center)
                    .height(Length::Units(20)),
                FontType::Text
                    .get_text("RTX 2060".to_string(), FontFamily::Kanit)
                    .size(ViewSize::Text.get_size(view)),
                FontType::TextBold
                    .get_text("400.00 €".to_string(), FontFamily::Kanit)
                    .horizontal_alignment(alignment::Horizontal::Right)
                    .size(ViewSize::Text.get_size(view))
                    .width(Length::Fill),
            ));
            column = column.push(row!(
                text("• ")
                    .size(ViewSize::Text.get_size(view))
                    .vertical_alignment(alignment::Vertical::Center)
                    .height(Length::Units(20)),
                FontType::Text
                    .get_text("RTX 2060".to_string(), FontFamily::Kanit)
                    .size(ViewSize::Text.get_size(view)),
                FontType::TextBold
                    .get_text("400.00 €".to_string(), FontFamily::Kanit)
                    .horizontal_alignment(alignment::Horizontal::Right)
                    .size(ViewSize::Text.get_size(view))
                    .width(Length::Fill),
            ));
            column = column.push(row!(
                text("• ")
                    .size(ViewSize::Text.get_size(view))
                    .vertical_alignment(alignment::Vertical::Center)
                    .height(Length::Units(20)),
                FontType::Text
                    .get_text("RTX 2060".to_string(), FontFamily::Kanit)
                    .size(ViewSize::Text.get_size(view)),
                FontType::TextBold
                    .get_text("400.00 €".to_string(), FontFamily::Kanit)
                    .horizontal_alignment(alignment::Horizontal::Right)
                    .size(ViewSize::Text.get_size(view))
                    .width(Length::Fill),
            ));
            column
        },
    ))
    .style(theme::Container::Custom(ContainerType::Box.get_box()))
    .padding(20)
    .width(Length::Fill)
}

fn beautify_legend(name: String, color: [f32; 3], view: &View) -> Column<'static, Message> {
    column!(row!(
        text(" ").width(Length::Units(20)),
        Canvas::new(LabelSquare::new(color))
            .width(Length::Units(ViewSize::Text.get_size(view)))
            .height(Length::Units(ViewSize::Text.get_size(view))),
        FontType::TextBold
            .get_text(name, FontFamily::Kanit)
            .size(ViewSize::Text.get_size(view)),
        text(" ").width(Length::Units(20)),
    )
    .spacing(10)
    .padding(5))
    .align_items(Alignment::Center)
}

fn spendings(profile: &Profile, view: &View) -> Container<'static, Message> {
    let rankings = profile.data.get_labels_rankings(Some(3));
    let _ = spendings_chart::draw(&rankings);

    container(column!(
        FontType::Title
            .get_text(
                "Dépenses par catégories".to_string(),
                FontFamily::IndieFlower
            )
            .width(Length::Fill)
            .style(Color::from([0.2235, 0.0, 0.5294]))
            .size(ViewSize::Title.get_size(view))
            .horizontal_alignment(alignment::Horizontal::Left),
        row!(
            container(svg(svg::Handle::from_path("assets/pie-chart.svg")))
                .width(Length::FillPortion(3)),
            container(
                {
                    let mut column = Column::new();
                    column = column.push(column!(text(" ")));
                    for ranking in rankings {
                        match ranking.0 {
                            Some(ranking) => {
                                column = column.push(beautify_legend(
                                    ranking.name.clone(),
                                    ranking.color,
                                    view,
                                ))
                            }
                            None => {
                                column = column.push(beautify_legend(
                                    "Autre".to_string(),
                                    [0.5, 0.5, 0.5],
                                    view,
                                ))
                            }
                        };
                    }
                    column = column.push(column!(text(" ")));
                    column
                }
                .width(Length::FillPortion(3))
            )
            .style(theme::Container::Custom(ContainerType::Box.get_box()))
        )
        .align_items(iced::Alignment::Center)
    ))
    .style(theme::Container::Custom(ContainerType::Box.get_box()))
    .padding(20)
}

fn pins(_profile: &Profile, view: &View) -> Container<'static, Message> {
    container(column!(
        FontType::Title
            .get_text("Rappels".to_string(), FontFamily::IndieFlower)
            .width(Length::Fill)
            .style(Color::from([0.2235, 0.0, 0.5294]))
            .size(ViewSize::Title.get_size(view))
            .horizontal_alignment(alignment::Horizontal::Left),
        {
            let mut column = Column::new().spacing(10);
            column = column.push(row!(
                svg(svg::Handle::from_path("assets/pin-fill.svg"))
                    .width(Length::Units(ViewSize::Header.get_size(&view) - 5))
                    .height(Length::Units(ViewSize::Header.get_size(&view) - 5)),
                FontType::Text
                    .get_text("RTX 2060".to_string(), FontFamily::Kanit)
                    .size(ViewSize::Text.get_size(view)),
            ));
            column = column.push(row!(
                svg(svg::Handle::from_path("assets/pin-fill.svg"))
                    .width(Length::Units(ViewSize::Header.get_size(&view) - 5))
                    .height(Length::Units(ViewSize::Header.get_size(&view) - 5)),
                FontType::Text
                    .get_text("RTX 2060".to_string(), FontFamily::Kanit)
                    .size(ViewSize::Text.get_size(view)),
            ));
            column = column.push(row!(
                svg(svg::Handle::from_path("assets/pin-fill.svg"))
                    .width(Length::Units(ViewSize::Header.get_size(&view) - 5))
                    .height(Length::Units(ViewSize::Header.get_size(&view) - 5)),
                FontType::Text
                    .get_text("RTX 2060".to_string(), FontFamily::Kanit)
                    .size(ViewSize::Text.get_size(view)),
            ));
            column
        },
    ))
    .style(theme::Container::Custom(ContainerType::Box.get_box()))
    .padding(20)
}
