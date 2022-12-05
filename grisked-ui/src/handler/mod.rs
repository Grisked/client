use iced::keyboard::{KeyCode, Modifiers};

use crate::{app::Grisked, entity::menu::MenuType};

mod save;
mod zoom;

pub fn handle_keys(keycode: KeyCode, modifiers: Modifiers, app: &mut Grisked) {
    if (keycode == KeyCode::Plus || keycode == KeyCode::Equals) && modifiers == Modifiers::CTRL {
        zoom::zoom_in(&mut app.view);
    }
    if (keycode == KeyCode::Minus || keycode == KeyCode::Key6) && modifiers == Modifiers::CTRL {
        zoom::zoom_out(&mut app.view);
    }
    if keycode == KeyCode::S && modifiers == Modifiers::CTRL {
        save::save(&app.profile);
    }
    if keycode == KeyCode::Escape {
        match app.menu_type {
            MenuType::AccountData(_) => app.menu_type = MenuType::Accounts,
            _ => {}
        }
    }
}
