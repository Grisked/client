use grisked_profile::profile::Profile;
use iced::{
    alignment, theme,
    widget::{column, button, text_input, container, row, text, Column, Container, Canvas},
    Length, Color, alignment::Alignment
};

use crate::{stylesheet::{ContainerType, label_square::LabelSquare}, view::View, Message, font::{FontFamily, FontType}};

pub fn accounts(profile: &Profile, view: &View) -> Container<'static, Message> {
    let center = Column::new().spacing(10).push(list_accounts(profile, view));

    let container: Container<Message> = container(row!(center).spacing(20))
        .height(Length::FillPortion(7))
        .width(Length::Fill)
        .padding(50)
        .into();
    container
}

fn list_accounts(_profile: &Profile, _view: &View) -> Container<'static, Message> {
        // Header
    container(column!(
        FontType::Title
            .get_text("Mes comptes".to_string(), FontFamily::IndieFlower)
            .width(Length::Fill)
            .style(Color::from([0.2235, 0.0, 0.5294]))
            .size(45)
            .horizontal_alignment(alignment::Horizontal::Center),
        // Selecteur de comptes
        row!(
        button(text("<").horizontal_alignment(alignment::Horizontal::Center).width(Length::Fill).size(30)).width(Length::FillPortion(1)),
        Canvas::new(LabelSquare::new([1.0, 0.0, 0.0])).width(Length::FillPortion(1)),
        Canvas::new(LabelSquare::new([1.0, 0.0, 0.0])).width(Length::FillPortion(1)),
        Canvas::new(LabelSquare::new([1.0, 0.0, 0.0])).width(Length::FillPortion(1)),
        button(text(">").horizontal_alignment(alignment::Horizontal::Center).width(Length::Fill).size(30)).width(Length::FillPortion(1)),
        ).spacing(10).align_items(Alignment::Center),
        // Weird wish spacing
        text(" "),
        text(" "),
        // Titres du haut
        row!(
            FontType::Title
                .get_text("Ajouter un compte".to_string(), FontFamily::IndieFlower)
                .width(Length::FillPortion(7))
                .style(Color::from([0.2235, 0.0, 0.5294]))
                .size(30)
                .horizontal_alignment(alignment::Horizontal::Center),
            FontType::Title
                .get_text("Créer un label".to_string(), FontFamily::IndieFlower)
                .width(Length::FillPortion(7))
                .style(Color::from([0.2235, 0.0, 0.5294]))
                .size(30)
                .horizontal_alignment(alignment::Horizontal::Center),
        ).spacing(50),
        row!(
            // Case en haut à gauche
            container(column!(
                row!(
                    container(column!(
                        container(column!(
                            text("Nom du compte"),
                        )).style(theme::Container::Custom(ContainerType::Box.get_box())).width(Length::FillPortion(7)).padding(10),
                        text(" "),
                        row!(
                            container(column!(
                                row!(
                                    text("Solde"),
                                    text("€").horizontal_alignment(alignment::Horizontal::Right).width(Length::Fill),
                                )
                            )).style(theme::Container::Custom(ContainerType::Box.get_box())).width(Length::FillPortion(1)).padding(10),
                            container(column!(
                                row!(
                                    text("Couleur").width(Length::Fill),
                                    Canvas::new(LabelSquare::new([1.0, 0.0, 0.0])).width(Length::Units(20)).height(Length::Units(20)),
                                    )
                            )).style(theme::Container::Custom(ContainerType::Box.get_box())).width(Length::FillPortion(1)).padding(10),
                        ).spacing(10),
                    )).width(Length::Fill).padding(10),
                    button("+"),
                ).align_items(Alignment::Center),
            )).style(theme::Container::Custom(ContainerType::Box.get_box())).width(Length::FillPortion(7)).padding(10),
            // Case au haut à droite
            container(column!(
                row!(
                    container(column!(
                        container(column!(
                            text("Nom du label"),
                        )).style(theme::Container::Custom(ContainerType::Box.get_box())).width(Length::FillPortion(7)).padding(10),
                        text(" "),
                        container(column!(
                            row!(
                                text("Couleur").width(Length::Fill),
                                Canvas::new(LabelSquare::new([1.0, 0.0, 0.0])).width(Length::Units(20)).height(Length::Units(20)),
                                )
                        )).style(theme::Container::Custom(ContainerType::Box.get_box())).width(Length::FillPortion(1)).padding(10),
                    )).width(Length::Fill).padding(10),
                    button("+"),
                ).align_items(Alignment::Center),
            )).style(theme::Container::Custom(ContainerType::Box.get_box())).width(Length::FillPortion(7)).padding(10),
        ).spacing(50),
        // Titres du bas
        row!(
            FontType::Title
                .get_text("Ajouter une facture".to_string(), FontFamily::IndieFlower)
                .width(Length::FillPortion(7))
                .style(Color::from([0.2235, 0.0, 0.5294]))
                .size(30)
                .horizontal_alignment(alignment::Horizontal::Center),
            FontType::Title
                .get_text("Ajouter une entrée".to_string(), FontFamily::IndieFlower)
                .width(Length::FillPortion(7))
                .style(Color::from([0.2235, 0.0, 0.5294]))
                .size(30)
                .horizontal_alignment(alignment::Horizontal::Center),
        ).spacing(50),
        row!(
            // Case en bas à gauche
            container(column!(
                row!(
                    container(column!(
                        row!(
                            container(column!(
                                text("Nom de la facture"),
                            )).style(theme::Container::Custom(ContainerType::Box.get_box())).width(Length::FillPortion(2)).padding(10),
                            container(column!(
                                row!(
                                    text("Montant"),
                                    text("€").horizontal_alignment(alignment::Horizontal::Right).width(Length::Fill),
                                    )
                            )).style(theme::Container::Custom(ContainerType::Box.get_box())).width(Length::FillPortion(1)).padding(10),
                        ).spacing(10),
                        text(" "),
                        row!(
                            container(column!(
                                text("DD/MM/YYYY"),
                            )).style(theme::Container::Custom(ContainerType::Box.get_box())).width(Length::FillPortion(2)).padding(10),
                            container(column!(
                                text("DD/MM/YYYY"),
                            )).style(theme::Container::Custom(ContainerType::Box.get_box())).width(Length::FillPortion(2)).padding(10),
                            container(column!(
                                text("Compte"),
                            )).style(theme::Container::Custom(ContainerType::Box.get_box())).width(Length::FillPortion(2)).padding(10),
                            container(column!(
                                text("Label"),
                            )).style(theme::Container::Custom(ContainerType::Box.get_box())).width(Length::FillPortion(2)).padding(10),
                        ).spacing(10),
                    )).width(Length::Fill).padding(10),
                    button("+"),
                ).align_items(Alignment::Center),
            )).style(theme::Container::Custom(ContainerType::Box.get_box())).width(Length::FillPortion(7)).padding(10),
            // Case en bas à droite
            container(column!(
                row!(
                    container(column!(
                        container(column!(
                            text("Nom de la facture"),
                        )).style(theme::Container::Custom(ContainerType::Box.get_box())).width(Length::FillPortion(7)).padding(10),
                        text(" "),
                        row!(
                            container(column!(
                                row!(
                                    text("Montant"),
                                    text("€").horizontal_alignment(alignment::Horizontal::Right).width(Length::Fill),
                                )
                            )).style(theme::Container::Custom(ContainerType::Box.get_box())).width(Length::FillPortion(1)).padding(10),
                            container(column!(
                                    text("DD/MM/YYYY").width(Length::Fill),
                            )).style(theme::Container::Custom(ContainerType::Box.get_box())).width(Length::FillPortion(1)).padding(10),
                        ).spacing(10),
                    )).width(Length::Fill).padding(10),
                    button("+"),
                ).align_items(Alignment::Center),
            )).style(theme::Container::Custom(ContainerType::Box.get_box())).width(Length::FillPortion(7)).padding(10),
        ).spacing(50),
    ))
    .padding(20)
}
