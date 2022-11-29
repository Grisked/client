use grisked_profile::profile::Profile;
use iced::{
    alignment, theme,
    widget::{button, column, container, row, text, Column, Container},
    Length,
};

use crate::{stylesheet::ContainerType, view::View, Message};

pub fn accounts(profile: &Profile, view: View) -> Container<Message> {
    let center = Column::new().spacing(10).push(list_accounts(profile, view));

    let container: Container<Message> = container(row!(center).spacing(20))
        .height(Length::FillPortion(7))
        .width(Length::Fill)
        .padding(50)
        .into();
    container
}

fn list_accounts(_profile: &Profile, _view: View) -> Container<Message> {
    container(column!(
        text("Comptes courants")
            .horizontal_alignment(alignment::Horizontal::Center)
            .width(Length::Fill)
            .size(30),
        row!(text(" ")),
        row!(
            text("Compte 1"),
            text("834€")
                .horizontal_alignment(alignment::Horizontal::Right)
                .width(Length::Fill)
        )
        .spacing(25),
        row!(
            text("Compte 2"),
            text("7493€")
                .horizontal_alignment(alignment::Horizontal::Right)
                .width(Length::Fill)
        )
        .spacing(25),
        row!(text(" ")),
        row!(text("Total courants: 45678€")
            .horizontal_alignment(alignment::Horizontal::Right)
            .width(Length::Fill)
            .size(15)),
    ))
    .style(theme::Container::Custom(ContainerType::Box.get_box()))
    .padding(20)
}
