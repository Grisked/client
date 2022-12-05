use iced::{
    widget::button::{Appearance, StyleSheet},
    Color, Theme, Vector,
};

#[derive(Default)]
pub struct IgnoredButton(pub [f32; 3]);

impl StyleSheet for IgnoredButton {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            text_color: Color::WHITE,
            background: None,
            border_radius: 360.0,
            border_width: 1150.0,
            border_color: Color::from_rgb(self.0[0], self.0[1], self.0[2]),
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }

    fn hovered(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            text_color: Color::WHITE,
            background: Some(iced::Background::Color(Color::from_rgba(
                self.0[0], self.0[1], self.0[2], 0.5,
            ))),
            border_radius: 360.0,
            border_width: 1150.0,
            border_color: Color::from_rgba(self.0[0], self.0[1], self.0[2], 0.8),
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }

    fn pressed(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            text_color: Color::WHITE,
            background: None,
            border_radius: 360.0,
            border_width: 1150.0,
            border_color: Color::from_rgb(self.0[0], self.0[1], self.0[2]),
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }

    fn disabled(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            text_color: Color::WHITE,
            background: None,
            border_radius: 360.0,
            border_width: 1150.0,
            border_color: Color::from_rgb(self.0[0], self.0[1], self.0[2]),
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }
}

#[derive(Default)]
pub struct EmptyButton;

impl StyleSheet for EmptyButton {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            text_color: Color::WHITE,
            background: None,
            border_radius: 360.0,
            border_width: 1150.0,
            border_color: Color::TRANSPARENT,
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }

    fn hovered(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            text_color: Color::WHITE,
            background: None,
            border_radius: 360.0,
            border_width: 1150.0,
            border_color: Color::TRANSPARENT,
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }

    fn pressed(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            text_color: Color::WHITE,
            background: None,
            border_radius: 360.0,
            border_width: 1150.0,
            border_color: Color::TRANSPARENT,
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }

    fn disabled(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            text_color: Color::WHITE,
            background: None,
            border_radius: 360.0,
            border_width: 1150.0,
            border_color: Color::TRANSPARENT,
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }
}
