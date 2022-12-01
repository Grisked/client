use grisked_profile::profile::Profile;
use iced::{
    alignment,
    alignment::Alignment,
    theme,
    widget::{button, column, container, row, svg, text, Canvas, Column, Container},
    Color, Length,
};

use crate::{
    font::{FontFamily, FontType},
    stylesheet::{label_square::LabelSquare, ButtonType, ContainerType},
    view::View,
    Message,
};

pub fn accounts(profile: &Profile, view: &View) -> Container<'static, Message> {
    let accounts = list_accounts(profile, view);
    let top = top_side(profile, view);
    let bottom = bottom_side(profile, view);

    let container: Container<Message> =
        container(column!(accounts, column!(top, bottom).spacing(50)))
            .height(Length::FillPortion(7))
            .width(Length::Fill)
            .padding(50);
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
            button(
                FontType::Title
                    .get_text("<".to_string(), FontFamily::Kanit)
                    .style(Color::BLACK)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .width(Length::Fill)
                    .size(100)
            )
            .style(theme::Button::Custom(ButtonType::BoxIgnored.get_box()))
            .width(Length::FillPortion(1)),
            button(container(column!(
                text("Compte 1")
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .width(Length::Fill)
                    .size(20),
                text("600.00 €")
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .width(Length::Fill)
                    .size(20),
            )))
            .style(theme::Button::Custom(
                ButtonType::AccountIgnored([0.0, 0.0, 1.0]).get_box()
            ))
            .width(Length::FillPortion(1)),
            button(container(column!(
                text("Compte 1")
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .width(Length::Fill)
                    .size(30),
                text("600.00 €")
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .width(Length::Fill)
                    .size(30),
            )))
            .style(theme::Button::Custom(
                ButtonType::AccountIgnored([0.0, 0.0, 1.0]).get_box()
            ))
            .width(Length::FillPortion(1)),
            button(container(column!(
                text("Compte 1")
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .width(Length::Fill)
                    .size(20),
                text("600.00 €")
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .width(Length::Fill)
                    .size(20),
            )))
            .style(theme::Button::Custom(
                ButtonType::AccountIgnored([0.0, 0.0, 1.0]).get_box()
            ))
            .width(Length::FillPortion(1)),
            button(
                FontType::Title
                    .get_text(">".to_string(), FontFamily::Kanit)
                    .style(Color::BLACK)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .width(Length::Fill)
                    .size(100)
            )
            .style(theme::Button::Custom(ButtonType::BoxIgnored.get_box()))
            .width(Length::FillPortion(1)),
        )
        .spacing(10)
        .align_items(Alignment::Center),
        // Weird wish spacing
        text(" "),
        text(" "),
        row!(text(" ")),
        row!(text(" ")),
    ))
}

fn top_side(profile: &Profile, view: &View) -> Column<'static, Message> {
    column!(
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
        )
        .spacing(50),
        row!(add_account(profile, view), add_label(profile, view)).spacing(50)
    )
}

fn bottom_side(profile: &Profile, view: &View) -> Column<'static, Message> {
    column!(
        // Titres du bas
        row!(
            FontType::Title
                .get_text("Ajouter une facture".to_string(), FontFamily::IndieFlower)
                .width(Length::FillPortion(7))
                .style(Color::from([0.2235, 0.0, 0.5294]))
                .size(30)
                .horizontal_alignment(alignment::Horizontal::Center),
            FontType::Title
                .get_text("Ajouter un revenu".to_string(), FontFamily::IndieFlower)
                .width(Length::FillPortion(7))
                .style(Color::from([0.2235, 0.0, 0.5294]))
                .size(30)
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .spacing(50),
        row!(add_invoice(profile, view), add_income(profile, view)).spacing(50)
    )
}

fn add_label(_profile: &Profile, _view: &View) -> Container<'static, Message> {
    // Case au haut à droite
    container(column!(row!(
        container(column!(
            container(column!(text("Nom du label"),))
                .style(theme::Container::Custom(ContainerType::Box.get_box()))
                .width(Length::FillPortion(7))
                .padding(10),
            text(" "),
            container(column!(row!(
                text("Couleur").width(Length::Fill),
                Canvas::new(LabelSquare::new([1.0, 0.0, 0.0]))
                    .width(Length::Units(20))
                    .height(Length::Units(20)),
            )))
            .style(theme::Container::Custom(ContainerType::Box.get_box()))
            .width(Length::FillPortion(1))
            .padding(10),
        ))
        .width(Length::Fill)
        .padding(10),
        button(svg(svg::Handle::from_path("assets/add_button.svg")))
            .style(theme::Button::Custom(ButtonType::BoxIgnored.get_box()))
            .width(Length::Units(48)),
    )
    .align_items(Alignment::Center),))
    .style(theme::Container::Custom(ContainerType::Box.get_box()))
    .width(Length::FillPortion(7))
    .padding(10)
}

fn add_account(_profile: &Profile, _view: &View) -> Container<'static, Message> {
    // Case en haut à gauche
    container(column!(row!(
        container(column!(
            container(column!(text("Nom du compte"),))
                .style(theme::Container::Custom(ContainerType::Box.get_box()))
                .width(Length::FillPortion(7))
                .padding(10),
            text(" "),
            row!(
                container(column!(row!(
                    text("Solde"),
                    text("€")
                        .horizontal_alignment(alignment::Horizontal::Right)
                        .width(Length::Fill),
                )))
                .style(theme::Container::Custom(ContainerType::Box.get_box()))
                .width(Length::FillPortion(1))
                .padding(10),
                container(column!(row!(
                    text("Couleur").width(Length::Fill),
                    Canvas::new(LabelSquare::new([1.0, 0.0, 0.0]))
                        .width(Length::Units(20))
                        .height(Length::Units(20)),
                )))
                .style(theme::Container::Custom(ContainerType::Box.get_box()))
                .width(Length::FillPortion(1))
                .padding(10),
            )
            .spacing(10),
        ))
        .width(Length::Fill)
        .padding(10),
        button(svg(svg::Handle::from_path("assets/add_button.svg")))
            .style(theme::Button::Custom(ButtonType::BoxIgnored.get_box()))
            .width(Length::Units(48)),
    )
    .align_items(Alignment::Center),))
    .style(theme::Container::Custom(ContainerType::Box.get_box()))
    .width(Length::FillPortion(7))
    .padding(10)
}

fn add_income(_profile: &Profile, _view: &View) -> Container<'static, Message> {
    // Case en bas à droite
    container(column!(row!(
        container(column!(
            container(column!(text("Nom de la facture"),))
                .style(theme::Container::Custom(ContainerType::Box.get_box()))
                .width(Length::FillPortion(7))
                .padding(10),
            text(" "),
            row!(
                container(column!(row!(
                    text("Montant"),
                    text("€")
                        .horizontal_alignment(alignment::Horizontal::Right)
                        .width(Length::Fill),
                )))
                .style(theme::Container::Custom(ContainerType::Box.get_box()))
                .width(Length::FillPortion(1))
                .padding(10),
                container(column!(text("DD/MM/YYYY").width(Length::Fill),))
                    .style(theme::Container::Custom(ContainerType::Box.get_box()))
                    .width(Length::FillPortion(1))
                    .padding(10),
            )
            .spacing(10),
        ))
        .width(Length::Fill)
        .padding(10),
        button(svg(svg::Handle::from_path("assets/add_button.svg")))
            .style(theme::Button::Custom(ButtonType::BoxIgnored.get_box()))
            .width(Length::Units(48)),
    )
    .align_items(Alignment::Center),))
    .style(theme::Container::Custom(ContainerType::Box.get_box()))
    .width(Length::FillPortion(7))
    .padding(10)
}

fn add_invoice(_profile: &Profile, _view: &View) -> Container<'static, Message> {
    // Case en bas à gauche
    container(column!(row!(
        container(column!(
            row!(
                container(column!(text("Nom de la facture"),))
                    .style(theme::Container::Custom(ContainerType::Box.get_box()))
                    .width(Length::FillPortion(2))
                    .padding(10),
                container(column!(row!(
                    text("Montant"),
                    text("€")
                        .horizontal_alignment(alignment::Horizontal::Right)
                        .width(Length::Fill),
                )))
                .style(theme::Container::Custom(ContainerType::Box.get_box()))
                .width(Length::FillPortion(1))
                .padding(10),
            )
            .spacing(10),
            text(" "),
            row!(
                container(column!(text("DD/MM/YYYY"),))
                    .style(theme::Container::Custom(ContainerType::Box.get_box()))
                    .width(Length::FillPortion(2))
                    .padding(10),
                container(column!(text("DD/MM/YYYY"),))
                    .style(theme::Container::Custom(ContainerType::Box.get_box()))
                    .width(Length::FillPortion(2))
                    .padding(10),
                container(column!(text("Label"),))
                    .style(theme::Container::Custom(ContainerType::Box.get_box()))
                    .width(Length::FillPortion(2))
                    .padding(10),
            )
            .spacing(10),
        ))
        .width(Length::Fill)
        .padding(10),
        button(svg(svg::Handle::from_path("assets/add_button.svg")))
            .style(theme::Button::Custom(ButtonType::BoxIgnored.get_box()))
            .width(Length::Units(48)),
    )
    .align_items(Alignment::Center),))
    .style(theme::Container::Custom(ContainerType::Box.get_box()))
    .width(Length::FillPortion(7))
    .padding(10)
}
