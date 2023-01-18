use iced::{
    alignment, theme,
    widget::{button, column, container, row, svg, text, Column, Container, Row},
    Color, Length,
};

use grisked_profile::{models::account::Account, profile::Profile};
use iced::{
    alignment::Alignment,
    widget::{text_input, Button, Canvas},
};

use crate::{
    entity::menu::MenuType,
    font::{FontFamily, FontType},
    stylesheet::{label_square::LabelSquare, ButtonType, ContainerType, TextInputType},
    view::View,
    FieldSettings, Message, UpdateBox,
};

use grisked_profile::models::bill::BillType;

use crate::{
    entity::accounts::{add_income, add_invoice},
    view::ViewSize,
};

pub fn deadlines(
    profile: &Profile,
    view: &View,
    field_settings: &FieldSettings,
) -> Container<'static, Message> {
    let center = Column::new()
        .spacing(10)
        .push(save_buttons(profile, view, field_settings));

    let container: Container<Message> = container(row!(center).spacing(20))
        .height(Length::FillPortion(7))
        .width(Length::Fill)
        .padding(50);
    container
}

fn save_buttons(
    _profile: &Profile,
    view: &View,
    field_settings: &FieldSettings,
) -> Container<'static, Message> {
    container(column!(
        container(column!(row!(
            container(column!(row!(
                container(container(column!(text_input(
                    "Nom",
                    &field_settings.invoice_name,
                    |m| { Message::UpdateBox(UpdateBox::InvoiceName(m)) }
                )
                .style(theme::TextInput::Custom(
                    TextInputType::Transparent.get_box()
                )))))
                .style(theme::Container::Custom(ContainerType::Box.get_box()))
                .width(Length::FillPortion(1))
                .padding(10),
                container(container(column!(text_input(
                    "Compte",
                    &field_settings.invoice_name,
                    |m| { Message::UpdateBox(UpdateBox::InvoiceName(m)) }
                )
                .style(theme::TextInput::Custom(
                    TextInputType::Transparent.get_box()
                )))))
                .style(theme::Container::Custom(ContainerType::Box.get_box()))
                .width(Length::FillPortion(1))
                .padding(10),
                container(column!(row!(
                    text_input("Montant", &field_settings.invoice_amount, |m| {
                        Message::UpdateBox(UpdateBox::InvoiceAmount(m))
                    })
                    .style(theme::TextInput::Custom(
                        TextInputType::Transparent.get_box()
                    ))
                    .width(Length::FillPortion(9)),
                    text("€")
                        .horizontal_alignment(alignment::Horizontal::Right)
                        .vertical_alignment(alignment::Vertical::Center)
                        .width(Length::FillPortion(1)),
                )))
                .style(theme::Container::Custom(ContainerType::Box.get_box()))
                .width(Length::FillPortion(1))
                .padding(10),
                container(column!(row!(
                    text_input("Mensualité", &field_settings.invoice_amount, |m| {
                        Message::UpdateBox(UpdateBox::InvoiceAmount(m))
                    })
                    .style(theme::TextInput::Custom(
                        TextInputType::Transparent.get_box()
                    ))
                    .width(Length::FillPortion(9)),
                    text("€")
                        .horizontal_alignment(alignment::Horizontal::Right)
                        .width(Length::FillPortion(1)),
                )))
                .style(theme::Container::Custom(ContainerType::Box.get_box()))
                .width(Length::FillPortion(1))
                .padding(10),
                container(container(column!(text_input(
                    "Date",
                    &field_settings.invoice_name,
                    |m| { Message::UpdateBox(UpdateBox::InvoiceName(m)) }
                )
                .style(theme::TextInput::Custom(
                    TextInputType::Transparent.get_box()
                )))))
                .style(theme::Container::Custom(ContainerType::Box.get_box()))
                .width(Length::FillPortion(1))
                .padding(10),
            )
            .spacing(20)))
            .width(Length::Fill)
            .padding(10),
            button(svg(svg::Handle::from_path("assets/add_button.svg")))
                .style(theme::Button::Custom(ButtonType::BoxIgnored.get_box()))
                .width(Length::Units(48))
                .on_press(Message::AddInvoice),
        )
        .align_items(Alignment::Center),))
        .style(theme::Container::Custom(ContainerType::Box.get_box()))
        .width(Length::FillPortion(7))
        .padding(10),
        row!(" "),
        container(column!(
            row!(
                FontType::TextBold
                    .get_text("Nom".to_string(), FontFamily::Kanit)
                    .size(ViewSize::Text.get_size(view))
                    .width(Length::FillPortion(3))
                    .horizontal_alignment(alignment::Horizontal::Center),
                FontType::TextBold
                    .get_text("Compte".to_string(), FontFamily::Kanit)
                    .size(ViewSize::Text.get_size(view))
                    .width(Length::FillPortion(3))
                    .horizontal_alignment(alignment::Horizontal::Center),
                FontType::TextBold
                    .get_text("Date".to_string(), FontFamily::Kanit)
                    .size(ViewSize::Text.get_size(view))
                    .width(Length::FillPortion(2))
                    .horizontal_alignment(alignment::Horizontal::Center),
                FontType::TextBold
                    .get_text("Montant".to_string(), FontFamily::Kanit)
                    .size(ViewSize::Text.get_size(view))
                    .width(Length::FillPortion(2))
                    .horizontal_alignment(alignment::Horizontal::Center),
                FontType::TextBold
                    .get_text("Restant".to_string(), FontFamily::Kanit)
                    .size(ViewSize::Text.get_size(view))
                    .width(Length::FillPortion(2))
                    .horizontal_alignment(alignment::Horizontal::Center),
                FontType::TextBold
                    .get_text("Mensualité".to_string(), FontFamily::Kanit)
                    .size(ViewSize::Text.get_size(view))
                    .width(Length::FillPortion(2))
                    .horizontal_alignment(alignment::Horizontal::Center),
                FontType::TextBold
                    .get_text("Temps restant".to_string(), FontFamily::Kanit)
                    .size(ViewSize::Text.get_size(view))
                    .width(Length::FillPortion(2))
                    .horizontal_alignment(alignment::Horizontal::Center),
                text(" ").width(Length::FillPortion(1))
            )
            .width(Length::Fill),
            row!(" "),
            row!(
                FontType::Text
                    .get_text("La fameuse RTX".to_string(), FontFamily::Kanit)
                    .size(ViewSize::Text.get_size(view))
                    .width(Length::FillPortion(3))
                    .horizontal_alignment(alignment::Horizontal::Center),
                FontType::Text
                    .get_text("Compte Commun".to_string(), FontFamily::Kanit)
                    .size(ViewSize::Text.get_size(view))
                    .width(Length::FillPortion(3))
                    .horizontal_alignment(alignment::Horizontal::Center),
                FontType::Text
                    .get_text("12/06/2023".to_string(), FontFamily::Kanit)
                    .size(ViewSize::Text.get_size(view))
                    .width(Length::FillPortion(2))
                    .horizontal_alignment(alignment::Horizontal::Center),
                FontType::Text
                    .get_text("500€".to_string(), FontFamily::Kanit)
                    .size(ViewSize::Text.get_size(view))
                    .width(Length::FillPortion(2))
                    .horizontal_alignment(alignment::Horizontal::Center),
                FontType::Text
                    .get_text("400€".to_string(), FontFamily::Kanit)
                    .size(ViewSize::Text.get_size(view))
                    .width(Length::FillPortion(2))
                    .horizontal_alignment(alignment::Horizontal::Center),
                FontType::Text
                    .get_text("100€".to_string(), FontFamily::Kanit)
                    .size(ViewSize::Text.get_size(view))
                    .width(Length::FillPortion(2))
                    .horizontal_alignment(alignment::Horizontal::Center),
                FontType::Text
                    .get_text("4 mois".to_string(), FontFamily::Kanit)
                    .size(ViewSize::Text.get_size(view))
                    .width(Length::FillPortion(2))
                    .horizontal_alignment(alignment::Horizontal::Center),
                button(svg(svg::Handle::from_path("assets/less_button.svg")))
                    .style(theme::Button::Custom(ButtonType::BoxIgnored.get_box()))
                    .width(Length::FillPortion(1))
                    .height(Length::Units(ViewSize::Text.get_size(view)))
                    .on_press(Message::AddLabel),
            ),
        ))
        .padding(20)
        .style(theme::Container::Custom(ContainerType::Box.get_box()))
    ))
    .padding(20)
}
