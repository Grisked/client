use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Label {
    pub id: u16,
    pub name: String,
    pub color: [f32; 3],
}

impl Label {
    pub fn new(id: u16, name: String, color: [f32; 3]) -> Self {
        for element in color {
            debug_assert!(
                (0.0..=1.0).contains(&element),
                "component must be on [0, 1]"
            );
        }
        Self { id, name, color }
    }
}
