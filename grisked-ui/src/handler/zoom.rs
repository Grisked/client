use iced::keyboard::{KeyCode, Modifiers};

use crate::view::View;

pub fn handle(keycode: KeyCode, modifiers: Modifiers, view: &mut View) {
    if (keycode == KeyCode::Plus || keycode == KeyCode::Equals) && modifiers == Modifiers::CTRL {
        view.upscale();
    }
    if keycode == KeyCode::Minus && modifiers == Modifiers::CTRL {
        view.downscale();
    }
}
