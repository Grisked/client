use grisked_profile::profile::Profile;
use iced::{
    alignment, theme,
    widget::{column, container, row, text, Column, Container},
    Color, Length,
};

use crate::{
    font::{FontFamily, FontType},
    stylesheet::ContainerType,
    view::{View, ViewSize},
    Message,
};

pub fn recent_accounts(profile: &Profile, view: View) -> Container<Message> {
    let container: Container<Message> = container(column!(
        FontType::Title
            .get_text("Comptes récents".to_string(), FontFamily::IndieFlower)
            .width(Length::Fill)
            .style(Color::from([0.2235, 0.0, 0.5294]))
            .size(ViewSize::Title.get_size(&view))
            .horizontal_alignment(alignment::Horizontal::Left),
        {
            let mut column = Column::new().spacing(25);
            for account in &profile.accounts {
                column = column.push(column!(
                    row!(
                        text("• ")
                            .size(ViewSize::Text.get_size(&view))
                            .vertical_alignment(alignment::Vertical::Center)
                            .height(Length::Units(25)),
                        FontType::TextBold
                            .get_text(account.name.clone(), FontFamily::Kanit)
                            .size(ViewSize::Text.get_size(&view)),
                        FontType::TextBold
                            .get_text(
                                format!("{:0.2} €", &account.get_account_balance()),
                                FontFamily::Kanit
                            )
                            .size(ViewSize::Text.get_size(&view))
                            .horizontal_alignment(alignment::Horizontal::Right)
                            .width(Length::Fill)
                    ),
                    {
                        let mut c_bills = Column::new();
                        for bill in &account.bills {
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
                                            text =
                                                text.style(Color::from([(189.0 / 255.0), 0.0, 0.0]))
                                        } else {
                                            text =
                                                text.style(Color::from([0.0, (134.0 / 255.0), 0.0]))
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
                ))
            }
            column
        },
    ))
    .style(theme::Container::Custom(ContainerType::Box.get_box()))
    .padding(20);
    container
}
