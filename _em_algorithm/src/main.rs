use std::fs::File;
use std::io::{BufRead, BufReader, Write};

mod matrix;
mod input;

fn main() {
    // // open csv file
    // let file = File::open("mnist_em.csv").expect("fail to open mnist_em.csv");
    // let reader = BufReader::new(file);

    // // open file for write
    // let mut file_for_input = File::create("src/input.rs").expect("fail to open input.rs");

    // for (index, line) in reader.lines().enumerate() {
    //     if index == 0 {
    //         file_for_input.write_all("use crate::matrix::*;\n".as_bytes()).unwrap();
    //         file_for_input.write_all("\n".as_bytes()).unwrap();
    //         continue;
    //     }
    //     let line = line.unwrap();

    //     // make starings and write
    //     file_for_input.write_all(format!("pub fn input{}() -> Matrix<f64> {{ \n", index).as_bytes()).unwrap();
    //     file_for_input.write_all(format!("  let v_int = vec![{}];\n", line).as_bytes()).unwrap();
    //     file_for_input.write_all(format!("  let mut v_norm: Vec<f64> = Vec::new();\n").as_bytes()).unwrap();
    //     file_for_input.write_all(format!("  for e in v_int {{ \n").as_bytes()).unwrap();
    //     file_for_input.write_all(format!("      v_norm.push(e as f64 / 255f64);\n").as_bytes()).unwrap();
    //     file_for_input.write_all(format!("  }}\n").as_bytes()).unwrap();
    //     file_for_input.write_all(format!("  Matrix::append(28, 28, v_norm)\n").as_bytes()).unwrap();
    //     file_for_input.write_all(format!("}}\n").as_bytes()).unwrap();
    //     file_for_input.write_all(format!("\n").as_bytes()).unwrap();
    // }
}
