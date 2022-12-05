use grisked_profile::profile::Profile;
use iced::keyboard::{KeyCode, Modifiers};

use crate::view::View;

mod save;
mod zoom;

pub fn handle(keycode: KeyCode, modifiers: Modifiers, view: &mut View, profile: &mut Profile) {
    if (keycode == KeyCode::Plus || keycode == KeyCode::Equals) && modifiers == Modifiers::CTRL {
        zoom::zoom_in(view);
    }
    if (keycode == KeyCode::Minus || keycode == KeyCode::Key6) && modifiers == Modifiers::CTRL {
        zoom::zoom_out(view);
    }
    if keycode == KeyCode::S && modifiers == Modifiers::CTRL {
        save::save(profile);
    }
}
