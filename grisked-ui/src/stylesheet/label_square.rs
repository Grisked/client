use iced::widget::canvas::{self, Cache, Cursor, Event, Geometry, Path};
use iced::{event::Status, mouse::Interaction, Rectangle, Theme};
use iced::{Color, Point};

use crate::Message;

pub struct LabelSquare {
    pub color: [f32; 3],
    square_cache: Cache,
}

impl LabelSquare {
    pub fn new(color: [f32; 3]) -> Self {
        Self {
            color,
            square_cache: Cache::new(),
        }
    }
}

impl canvas::Program<Message> for LabelSquare {
    type State = Interaction;

    fn update(
        &self,
        _state: &mut Self::State,
        _event: Event,
        _bounds: Rectangle<f32>,
        _cursor: Cursor,
    ) -> (Status, Option<Message>) {
        (Status::Captured, None)
    }

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: Rectangle<f32>,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let square = self.square_cache.draw(bounds.size(), |frame| {
            let background = Path::rectangle(Point::ORIGIN, frame.size());
            frame.fill(
                &background,
                Color {
                    r: self.color[0],
                    g: self.color[1],
                    b: self.color[2],
                    a: 1.0,
                },
            )
        });
        vec![square]
    }
}
