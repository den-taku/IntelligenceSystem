use crate::matrix::*;

pub struct EM<T>{
    data: Vec<Matrix<T>>
}

impl<T> EM<T> {
    pub fn new(data: Vec<Matrix<T>>) -> Self {
        EM {
            data
        }
    }
}
