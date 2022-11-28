use iced::{widget::container::Appearance, Background, Color, Theme};

const BOX_APPEARANCE: Appearance = Appearance {
    text_color: Some(Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    }),
    background: Some(Background::Color(Color {
        r: (232.0 / 255.0),
        g: (232.0 / 255.0),
        b: (232.0 / 255.0),
        a: 1.0,
    })),
    border_radius: 8.0,
    border_width: 2.0,
    border_color: Color {
        r: 0.0,
        g: (68.0 / 255.0),
        b: (117.0 / 255.0),
        a: 1.0,
    },
};

pub fn get_appearance<'a>(_theme: &'a Theme) -> Appearance {
    BOX_APPEARANCE
}
