use crate::matrix::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Div;
use std::str::FromStr;

pub fn split_data_at<T: Clone>(index: usize, data: Vec<Matrix<T>>) -> (Vec<Matrix<T>>, Vec<Matrix<T>>) {
   (data[..index].to_vec(), data[index..].to_vec()) 
}

pub fn normalize_data<T>(data: Vec<Matrix<T>>, norm: T) -> Vec<Matrix<T>>
where
    T: Div<Output = T> + Clone + Copy,
{
    let data: Vec<Matrix<T>> = data.iter().map(|e| e / norm).collect();
    data
}

pub fn read_csv<T>(filename: &str) -> Vec<Matrix<T>>
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

#[cfg(test)]
mod tests_em {
    use super::*;

    #[test]
    fn for_read_csv() {
        let data = read_csv::<f64>("data/mnist_em.csv");
        assert!(data.iter().all(|m| m.len() == 28 * 28));
        assert_eq!(data.len(), 21770);
    }

    #[test]
    fn for_normalize_data() {
        let data = read_csv::<f64>("data/mnist_em.csv");
        let data = normalize_data(data, 255f64);
        assert!(data.iter().all(|m| m.len() == 28 * 28));
        assert_eq!(data.len(), 21770);
    }
}
