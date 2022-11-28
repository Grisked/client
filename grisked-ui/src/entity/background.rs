use iced::{widget::container::Appearance, Background, Color, Theme};

const BOX_APPEARANCE: Appearance = Appearance {
    text_color: Some(Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    }),
    background: Some(Background::Color(Color {
        r: (210.0 / 255.0),
        g: (210.0 / 255.0),
        b: (210.0 / 255.0),
        a: 1.0,
    })),
    border_radius: 0.0,
    border_width: 0.0,
    border_color: Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    },
};

pub fn get_appearance<'a>(_theme: &'a Theme) -> Appearance {
    BOX_APPEARANCE
}
