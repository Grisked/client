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
    view::{View, ViewSize},
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

fn bottom_side(_profile: &Profile, view: &View) -> Column<'static, Message> {
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
                        .get_text("1073.00€".to_string(), FontFamily::Kanit)
                        .width(Length::Fill)
                        .style(Color::from([0.0, (134.0 / 255.0), 0.0]))
                        .size(ViewSize::Title.get_size(view))
                        .horizontal_alignment(alignment::Horizontal::Right),
                )
                .padding([0, 0, 10, 0]),
                {
                    let mut column = Column::new();

                    for _ in 0..10 {
                        column = column.push(
                            row!(
                                FontType::Text
                                    .get_text("[MonLabel]".to_string(), FontFamily::Kanit)
                                    .size(ViewSize::Text.get_size(view)),
                                FontType::TextBold
                                    .get_text("Facture du facteur".to_string(), FontFamily::Kanit)
                                    .size(ViewSize::Text.get_size(view)),
                                FontType::Text
                                    .get_text("09/12/2004".to_string(), FontFamily::Kanit)
                                    .size(ViewSize::Text.get_size(view)),
                                svg(svg::Handle::from_path("assets/arrow-right-short.svg"))
                                    .height(Length::Units(ViewSize::Text.get_size(view)))
                                    .width(Length::Units(ViewSize::Text.get_size(view))),
                                FontType::Text
                                    .get_text("10/01/2005".to_string(), FontFamily::Kanit)
                                    .size(ViewSize::Text.get_size(view)),
                                FontType::TextBold
                                    .get_text("1139.12€".to_string(), FontFamily::Kanit)
                                    .size(ViewSize::Text.get_size(view)),
                            )
                            .spacing(10),
                        );
                    }

                    column
                },
                row!(FontType::Title
                    .get_text("Revenus".to_string(), FontFamily::IndieFlower)
                    .width(Length::Fill)
                    .style(Color::from([0.2235, 0.0, 0.5294]))
                    .size(ViewSize::Title.get_size(view)),)
                .padding([10, 0, 10, 0]),
                row!(
                    FontType::TextBold
                        .get_text("La mère grand met bien".to_string(), FontFamily::Kanit)
                        .size(ViewSize::Text.get_size(view)),
                    FontType::Text
                        .get_text("09/12/2004".to_string(), FontFamily::Kanit)
                        .size(ViewSize::Text.get_size(view)),
                    FontType::TextBold
                        .get_text("1139.12€".to_string(), FontFamily::Kanit)
                        .style(Color::from([0.0, (134.0 / 255.0), 0.0]))
                        .size(ViewSize::Text.get_size(view)),
                )
                .spacing(10)
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
