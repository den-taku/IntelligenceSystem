use em_algorithm::matrix::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::ops::Div;

fn main() {
    // read data from mnist_em.csv, which has 21770 handwritten characters' images that consists of 28x28 pixels.
    let data = read_csv::<f64>("mnist_em.csv");
    assert!(data.iter().all(|m| m.len() == 28 * 28));
    assert_eq!(data.len(), 21770);

    // normalize data
    let data = normalize_data(data, 255f64);
    assert!(data.iter().all(|m| m.len() == 28 * 28));
    assert_eq!(data.len(), 21770);
}

fn normalize_data<T>(data: Vec<Matrix<T>>, norm: T) -> Vec<Matrix<T>>
where 
    T: Div<Output = T> + Clone + Copy
{
    let data: Vec<Matrix<T>> = data.iter().map(|e| e / norm).collect();
    data
}

fn read_csv<T>(filename: &str) -> Vec<Matrix<T>>
where
    T: Clone + FromStr,
{
    let file = File::open(filename).expect(&format!("fail to open {}", filename));
    let reader = BufReader::new(file);

    let mut data = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        if index == 0 {
            // first line is for title
            continue;
        }
        let line = line.unwrap();
        let v: Vec<T> = line
            .split(',')
            .map(|s| T::from_str(s))
            .map(|r| {
                if let Ok(e) = r {
                    e
                } else {
                    panic!("fail to change type.")
                }
            })
            .collect();
        data.push(Matrix::append(28, 28, v));
    }
    data
}
