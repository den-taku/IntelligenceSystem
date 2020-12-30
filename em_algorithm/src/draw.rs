use crate::matrix::*;
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
    encoder.encode(&pixels, 28u32, 28u32, ColorType::L8).expect("encode error.");

    Ok(())
}
