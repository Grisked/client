use iced::{
    alignment, theme,
    widget::{column, container, row, svg, Column, Container, Row},
    Color, Length,
};

use grisked_profile::{
    models::{account::Account, bill::BillType},
    profile::Profile,
};

use crate::{
    font::{FontFamily, FontType},
    stylesheet::ContainerType,
    view::{View, ViewSize},
    FieldSettings, Message,
};

pub fn account_data(
    profile: &Profile,
    view: &View,
    field_settings: &FieldSettings,
    account: &Account,
) -> Container<'static, Message> {
    let top = top_side(view, field_settings, account);
    let bottom = bottom_side(profile, view, account);

    let container: Container<Message> = container(column!(column!(top, bottom).spacing(50)))
        .height(Length::FillPortion(7))
        .width(Length::Fill)
        .padding(50);
    container
}

fn top_side(
    _view: &View,
    _field_settings: &FieldSettings,
    account: &Account,
) -> Column<'static, Message> {
    column!(FontType::Title
        .get_text(account.name.clone(), FontFamily::IndieFlower)
        .width(Length::Fill)
        .style(Color::from([0.2235, 0.0, 0.5294]))
        .size(50)
        .horizontal_alignment(alignment::Horizontal::Center),)
}

fn get_incomes(view: &View, account: &Account) -> Column<'static, Message> {
    let mut column = Column::new();

    for invoice in account.get_bills() {
        if invoice.bill_type != BillType::Income {
            continue;
        }
        let mut row = Row::new().spacing(10);

        row = row.push(
            FontType::TextBold
                .get_text(invoice.name.clone(), FontFamily::Kanit)
                .size(ViewSize::Text.get_size(view)),
        );

        row = row.push(
            FontType::Text
                .get_text("09/12/2004".to_string(), FontFamily::Kanit)
                .size(ViewSize::Text.get_size(view)),
        );
        row = row.push(
            FontType::TextBold
                .get_text(invoice.pretty_price(), FontFamily::Kanit)
                .style(Color::from([0.0, (134.0 / 255.0), 0.0]))
                .size(ViewSize::Text.get_size(view)),
        );

        column = column.push(row);
    }

    column
}

fn get_invoices(profile: &Profile, view: &View, account: &Account) -> Column<'static, Message> {
    let mut column = Column::new();

    for invoice in account.get_bills() {
        if invoice.bill_type != BillType::Invoice {
            continue;
        }
        let mut row = Row::new().spacing(10);

        match profile.data.get_label_by_id(invoice.label_id) {
            Some(label) => {
                row = row.push(
                    FontType::Text
                        .get_text(format!("[{}]", label.name), FontFamily::Kanit)
                        .size(ViewSize::Text.get_size(view)),
                )
            }
            None => {
                row = row.push(
                    FontType::Text
                        .get_text("[]".to_string(), FontFamily::Kanit)
                        .size(ViewSize::Text.get_size(view)),
                )
            }
        }
        row = row.push(
            FontType::TextBold
                .get_text(invoice.name.clone(), FontFamily::Kanit)
                .size(ViewSize::Text.get_size(view)),
        );
        row = row.push(
            FontType::Text
                .get_text("09/12/2004".to_string(), FontFamily::Kanit)
                .size(ViewSize::Text.get_size(view)),
        );
        row = row.push(
            svg(svg::Handle::from_path("assets/arrow-right-short.svg"))
                .height(Length::Units(ViewSize::Text.get_size(view)))
                .width(Length::Units(ViewSize::Text.get_size(view))),
        );
        row = row.push(
            FontType::Text
                .get_text("10/01/2005".to_string(), FontFamily::Kanit)
                .size(ViewSize::Text.get_size(view)),
        );
        row = row.push(
            FontType::TextBold
                .get_text(invoice.pretty_price(), FontFamily::Kanit)
                .size(ViewSize::Text.get_size(view)),
        );
        column = column.push(row);
    }

    column
}

fn bottom_side(profile: &Profile, view: &View, account: &Account) -> Column<'static, Message> {
    let color = if account.get_account_balance() > 0.0 {
        Color::from([0.0, (134.0 / 255.0), 0.0])
    } else {
        Color::from([(189.0 / 255.0), 0.0, 0.0])
    };

    column!(
        // Titres du bas
        row!(
            container(column!(
                row!(
                    FontType::Title
                        .get_text("Factures".to_string(), FontFamily::IndieFlower)
                        .width(Length::Fill)
                        .style(Color::from([0.2235, 0.0, 0.5294]))
                        .size(ViewSize::Title.get_size(view)),
                    FontType::TextBold
                        .get_text(account.pretty_account_balance(), FontFamily::Kanit)
                        .width(Length::Fill)
                        .style(color)
                        .size(ViewSize::Title.get_size(view))
                        .horizontal_alignment(alignment::Horizontal::Right),
                )
                .padding([0, 0, 10, 0]),
                get_invoices(profile, view, account),
                row!(FontType::Title
                    .get_text("Revenus".to_string(), FontFamily::IndieFlower)
                    .width(Length::Fill)
                    .style(Color::from([0.2235, 0.0, 0.5294]))
                    .size(ViewSize::Title.get_size(view)),)
                .padding([10, 0, 10, 0]),
                get_incomes(view, account)
            ))
            .style(theme::Container::Custom(ContainerType::Box.get_box()))
            .padding(10)
            .width(Length::FillPortion(2)),
            container(
                FontType::Title
                    .get_text("Ajouter un revenu".to_string(), FontFamily::IndieFlower)
                    .width(Length::FillPortion(7))
                    .style(Color::from([0.2235, 0.0, 0.5294]))
                    .size(30)
                    .horizontal_alignment(alignment::Horizontal::Center),
            )
            .style(theme::Container::Custom(ContainerType::Box.get_box()))
            .width(Length::FillPortion(1)),
        )
        .spacing(50),
    )
}
