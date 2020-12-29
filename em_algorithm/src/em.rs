use crate::matrix::*;

#[derive(Debug, Clone)]
pub struct EM<T>{
    mixed_number: usize,
    variance: T,
    data: Vec<Matrix<T>>
}

impl<T> EM<T> {
    pub fn new(mixed_number: usize, variance: T, data: Vec<Matrix<T>>) -> Self {
        EM {
            mixed_number,
            variance,
            data
        }
    }
}

impl<T> EM<T> {
    pub fn mixed_number(&self) -> usize {
        self.mixed_number
    }
}

impl<T: Clone> EM<T> {
    pub fn variance(&self) -> T {
        self.variance.clone()
    }
}
