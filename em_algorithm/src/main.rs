mod matrix;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use matrix::*;

fn main() {
    let file = File::open("mnist_em.csv").expect("fail to open mnist_em.csv");
    let reader = BufReader::new(file);

    let mut data = Vec::new();
    
    for (index, line) in reader.lines().enumerate() {
        if index == 0 {
            continue;
        }
        let line = line.unwrap();
        let v: Vec<f64> = line.split(',').map(|s| f64::from_str(s).unwrap()).collect();
        data.push(Matrix::append(28, 28, v));
    }

    // println!("{:?}", data);
}
