use grisked_profile::{
    models::{account::Account, label::Label},
    profile::Profile,
};
use iced::{
    alignment,
    alignment::Alignment,
    theme,
    widget::{
        button, column, container, row, svg, text, text_input, Button, Canvas, Column, Container,
        Row,
    },
    Color, Length,
};

use crate::{
    entity::menu::MenuType,
    font::{FontFamily, FontType},
    stylesheet::{label_square::LabelSquare, ButtonType, ContainerType},
    view::View,
    FieldSettings, Message, UpdateBox,
};

pub fn accountsdata(
    profile: &Profile,
    view: &View,
    field_settings: &FieldSettings,
    _account: &Account,
) -> Container<'static, Message> {
    let top = top_side(profile, view, field_settings);
    let bottom = bottom_side(profile, view);

    let container: Container<Message> = container(column!(column!(top, bottom).spacing(50)))
        .height(Length::FillPortion(7))
        .width(Length::Fill)
        .padding(50);
    container
}

fn top_side(
    _profile: &Profile,
    _view: &View,
    _field_settings: &FieldSettings,
) -> Column<'static, Message> {
    column!(FontType::Title
        .get_text("Compte Lambda".to_string(), FontFamily::IndieFlower)
        .width(Length::Fill)
        .style(Color::from([0.2235, 0.0, 0.5294]))
        .size(50)
        .horizontal_alignment(alignment::Horizontal::Center),)
}

fn bottom_side(_profile: &Profile, _view: &View) -> Column<'static, Message> {
    column!(
        // Titres du bas
        row!(
            container(row!(
                FontType::Text
                    .get_text("Factures".to_string(), FontFamily::IndieFlower)
                    .width(Length::Fill)
                    .style(Color::from([0.2235, 0.0, 0.5294]))
                    .size(30),
                FontType::TextBold
                    .get_text("1073.00€".to_string(), FontFamily::Kanit)
                    .width(Length::Fill)
                    .style(Color::from([0.0, (134.0 / 255.0), 0.0]))
                    .size(30)
                    .horizontal_alignment(alignment::Horizontal::Right),
            ))
            .style(theme::Container::Custom(ContainerType::Box.get_box()))
            .padding(10)
            .width(Length::FillPortion(1)),
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
