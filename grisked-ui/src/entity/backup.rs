use grisked_profile::profile::Profile;
use iced::{
    alignment,
    widget::{button, column, container, row, text, Column, Container},
    Length,
};

use crate::{view::View, Message};

pub fn backup(profile: &Profile, view: &View) -> Container<'static, Message> {
    let center = Column::new().spacing(10).push(save_buttons(profile, view));

    let container: Container<Message> = container(row!(center).spacing(20))
        .height(Length::FillPortion(7))
        .width(Length::Fill)
        .padding(50);
    container
}

fn save_buttons(_profile: &Profile, _view: &View) -> Container<'static, Message> {
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
    .padding(20)
}
