use iced::{
    widget::button::{Appearance, StyleSheet},
    Color, Theme, Vector,
};

const SELECTED_APPEARANCE: Appearance = Appearance {
    text_color: Color {
        r: 255.0,
        g: (243.0 / 255.0),
        b: 255.0,
        a: 1.0,
    },
    background: None,
    border_radius: 0.0,
    border_width: 0.0,
    border_color: Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    },
    shadow_offset: Vector::new(0.0, 0.0),
};

const IGNORED_APPEARANCE: Appearance = Appearance {
    text_color: Color {
        r: 255.0,
        g: (243.0 / 255.0),
        b: 255.0,
        a: 0.4,
    },
    background: None,
    border_radius: 0.0,
    border_width: 0.0,
    border_color: Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    },
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
