use iced::{
    widget::container::{Appearance, StyleSheet},
    Background, Color, Theme,
};

pub struct AppBackground;

impl StyleSheet for AppBackground {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> Appearance {
        Appearance {
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
        }
    }
}
