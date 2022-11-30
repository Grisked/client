use grisked_profile::models::label::Label;

use plotters::{prelude::*, style::full_palette::GREY};

const OUT_FILE_NAME: &'static str = "assets/pie-chart.svg";

pub fn draw(infos: &Vec<(Option<Label>, f64)>) -> Result<(), Box<dyn std::error::Error>> {
    let root_area = SVGBackend::new(&OUT_FILE_NAME, (1280, 720)).into_drawing_area();
    root_area.fill(&TRANSPARENT).unwrap();

    let dims = root_area.dim_in_pixel();
    let center = (dims.0 as i32 / 2, dims.1 as i32 / 2);
    let radius = 300.0;
    let sizes: Vec<f64> = infos.iter().map(|(_, value)| value.clone()).collect();
    let colors: Vec<RGBColor> = infos
        .iter()
        .map(|(value, _)| match value {
            Some(label) => RGBColor(
                (label.color[0] * 255.0) as u8,
                (label.color[1] * 255.0) as u8,
                (label.color[2] * 255.0) as u8,
            ),
            None => GREY,
        })
        .collect();
    let labels: Vec<&str> = infos.iter().map(|_| "").collect();

    let mut pie = Pie::new(&center, &radius, &sizes, &colors, &labels);
    pie.start_angle(270.0);
    root_area.draw(&pie)?;

    Ok(())
}
