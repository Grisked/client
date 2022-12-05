use crate::view::View;

pub fn zoom_in(view: &mut View) {
    view.upscale();
}

pub fn zoom_out(view: &mut View) {
    view.downscale();
}
