use crate::matrix::*;
use gnuplot::*;
use image::png::PngEncoder;
use image::ColorType;
use std::fs::File;

pub fn write_image<F: Clone + FromPrimitive + Float>(
    filename: &str,
    parameter: Matrix<F>,
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;

    let pixels = parameter
        .to_vec()
        .iter()
        .map(|e| (*e * F::from_f64(255.0).unwrap()).to_u8().unwrap())
        .collect::<Vec<u8>>();

    let encoder = PngEncoder::new(output);
    encoder
        .encode(&pixels, 28u32, 28u32, ColorType::L8)
        .expect("encode error.");

    Ok(())
}

pub fn draw_graph(
    x_range_from: f64,
    x_range_to: f64,
    y_range_from: f64,
    y_range_to: f64,
    x_label: &str,
    y_label: &str,
    color: &str,
    data: Vec<(f64, f64)>,
) {
    let mut fg = Figure::new();
    {
        let axec = fg
            .axes2d()
            .set_x_axis(true, &[])
            .set_x_range(Fix(x_range_from), Fix(x_range_to))
            .set_y_range(Fix(y_range_from), Fix(y_range_to))
            .set_x_label(x_label, &[])
            .set_y_label(y_label, &[]);
        data.iter().fold((), |_, e| {
            axec.points(&[e.0], &[e.1], &[Color(color), PointSymbol('O')]);
        })
    }
    let _ = fg.show();
}

pub fn draw_graph_log10(
    x_range_from: f64,
    x_range_to: f64,
    y_range_from: f64,
    y_range_to: f64,
    x_label: &str,
    y_label: &str,
    color: &str,
    data: Vec<(f64, f64)>,
) {
    let mut fg = Figure::new();
    {
        let axec = fg
            .axes2d()
            .set_x_axis(true, &[])
            .set_x_range(Fix(x_range_from), Fix(x_range_to))
            .set_y_range(Fix(y_range_from), Fix(y_range_to))
            .set_y_log(Some(10.0))
            .set_x_label(x_label, &[])
            .set_y_label(y_label, &[]);
        data.iter().fold((), |_, e| {
            axec.points(&[e.0], &[e.1], &[Color(color), PointSymbol('O')]);
        })
    }
    let _ = fg.show();
}
