use crate::matrix::*;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct EM<T>{
    mixed_number: usize,
    variance: T,
    allowable_error: f64,
    data: Vec<Matrix<T>>,
    parameters: Vec<Matrix<T>>
}

impl<T: Clone> EM<T> {
    // judge convergency, too?
    pub fn estimate(&mut self) -> Vec<Matrix<T>> {
        while {
            self.expect();
            self.maximize()
        } {}
        self.parameters()
    }
}

impl<T> EM<T> {
    fn expect(&mut self) {
        unimplemented!()
    }
}

impl<T> EM<T> {
    fn maximize(&mut self) -> bool {
        unimplemented!()
    }
}

impl<T> EM<T> {
    fn judge_convergence(&self, new_parameters: Vec<Matrix<T>>) -> bool {
        unimplemented!()
    }
}

impl<T: Clone> EM<T> {
    pub fn new(mixed_number: usize, variance: T, allowable_error: f64, data: Vec<Matrix<T>>) -> Self {
        // choose initial parameters.
        let mut rng = rand::thread_rng();

        let mut parameters = Vec::new();
        for _ in 0..mixed_number {
            let index = rng.gen::<usize>() % data.len();
            parameters.push((*data.index(index)).clone());
        }

        EM {
            mixed_number,
            variance,
            allowable_error,
            data,
            parameters
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
    pub fn parameters(&self) -> Vec<Matrix<T>> {
        self.parameters.clone()
    }
}
