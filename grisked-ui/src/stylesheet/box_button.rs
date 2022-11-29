use iced::{
    widget::button::{Appearance, StyleSheet},
    Color, Theme, Vector,
};

const SELECTED_APPEARANCE: Appearance = Appearance {
    text_color: Color::TRANSPARENT,
    background: Some(iced::Background::Color(Color {
        r: (180.0 / 255.0),
        g: (180.0 / 255.0),
        b: (180.0 / 255.0),
        a: 0.2,
    })),
    border_radius: 0.0,
    border_width: 0.0,
    border_color: Color::TRANSPARENT,
    shadow_offset: Vector::new(0.0, 0.0),
};

const IGNORED_APPEARANCE: Appearance = Appearance {
    text_color: Color::TRANSPARENT,
    background: None,
    border_radius: 0.0,
    border_width: 0.0,
    border_color: Color::TRANSPARENT,
    shadow_offset: Vector::new(0.0, 0.0),
};

#[derive(Default)]
pub struct SelectedButton;

impl StyleSheet for SelectedButton {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> Appearance {
        SELECTED_APPEARANCE
    }

    fn hovered(&self, _style: &Self::Style) -> Appearance {
        SELECTED_APPEARANCE
    }

    fn pressed(&self, _style: &Self::Style) -> Appearance {
        SELECTED_APPEARANCE
    }

    fn disabled(&self, _style: &Self::Style) -> Appearance {
        SELECTED_APPEARANCE
    }
}

#[derive(Default)]
pub struct IgnoredButton;

impl StyleSheet for IgnoredButton {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> Appearance {
        IGNORED_APPEARANCE
    }

    fn hovered(&self, _style: &Self::Style) -> Appearance {
        SELECTED_APPEARANCE
    }

    fn pressed(&self, _style: &Self::Style) -> Appearance {
        IGNORED_APPEARANCE
    }

    fn disabled(&self, _style: &Self::Style) -> Appearance {
        IGNORED_APPEARANCE
    }
}
