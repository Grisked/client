use plotters::{
    prelude::*,
    style::full_palette::{ORANGE, PURPLE},
};

const OUT_FILE_NAME: &'static str = "assets/pie-chart.svg";

pub fn draw() -> Result<(), Box<dyn std::error::Error>> {
    let root_area = SVGBackend::new(&OUT_FILE_NAME, (1280, 720)).into_drawing_area();
    root_area.fill(&TRANSPARENT).unwrap();

    let dims = root_area.dim_in_pixel();
    let center = (dims.0 as i32 / 2, dims.1 as i32 / 2);
    let radius = 300.0;
    let sizes = vec![36.0, 29.0, 23.0, 12.0];
    let _rgba = RGBAColor(0, 50, 255, 1.0);
    let colors = vec![BLUE, GREEN, ORANGE, PURPLE];
    let labels = vec!["", "", "", ""];

    let mut pie = Pie::new(&center, &radius, &sizes, &colors, &labels);
    pie.start_angle(270.0);
    pie.label_style((("sans-serif", 50).into_font()).color(&(ORANGE)));
    pie.percentages((("sans-serif", radius * 0.08).into_font()).color(&WHITE));
    root_area.draw(&pie)?;

    Ok(())
}
