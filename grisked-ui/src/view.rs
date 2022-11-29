#[derive(Clone)]
pub struct View {
    zoom_modifier: f32,
}

impl Default for View {
    fn default() -> Self {
        Self { zoom_modifier: 1.0 }
    }
}

impl View {
    pub fn get_size(&self, default_size: u16) -> u16 {
        (self.zoom_modifier * default_size as f32) as u16
    }

    pub fn upscale(&mut self) {
        if self.zoom_modifier <= 1.8 {
            self.zoom_modifier += 0.4;
        }
    }

    pub fn downscale(&mut self) {
        if self.zoom_modifier >= 1.4 {
            self.zoom_modifier -= 0.4;
        }
    }
}

pub enum ViewSize {
    Header,
    Title,
    Text,
}

impl ViewSize {
    pub fn get_size(&self, view: &View) -> u16 {
        match self {
            Self::Header => view.get_size(26),
            Self::Title => view.get_size(30),
            Self::Text => view.get_size(20),
        }
    }
}
