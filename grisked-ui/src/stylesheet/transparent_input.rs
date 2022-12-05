use iced::{
    widget::text_input::{Appearance, StyleSheet},
    Background, Color, Theme,
};

pub struct TransparentInput;

impl StyleSheet for TransparentInput {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            background: Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.0)),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }

    fn focused(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            background: Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.2)),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::BLACK,
        }
    }

    fn hovered(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            background: Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.0)),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::BLACK,
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgba(0.2, 0.2, 0.2, 0.8)
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgba(0.0, 0.0, 0.0, 1.0)
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgba(0.0, 1.0, 0.0, 0.8)
    }
}
