#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use grisked_ui::app;

pub fn main() {
    let _ = app::launch();
}
