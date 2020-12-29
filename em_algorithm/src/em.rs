use crate::matrix::*;
use num_traits::Float;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct EM<F>{
    mixed_number: usize,
    variance: F,
    allowable_error: F,
    data: Vec<Matrix<F>>,
    parameters: Vec<Matrix<F>>
}

impl<F> EM<F> 
where
    F: Clone + Float + FromPrimitive
{
    pub fn estimate(&mut self) -> Vec<Matrix<F>> {
        while {
            self.expect();
            self.maximize()
        } {}
        self.parameters()
    }
}

impl<F> EM<F> 
where
    F: Float + FromPrimitive + ToPrimitive
{
    // Expectation step
    fn expect(&mut self) {
        unimplemented!()
    }

    pub fn calcurate_p(&self, t: usize, i: usize) -> F {
        let x: f64 = -((&self.data[t] - &self.parameters()[i]).norm2_row::<f64>() / 2.0f64 / (self.variance()).to_f64().unwrap() / (self.variance()).to_f64().unwrap());
        F::from_f64((x).exp()).unwrap()
    }
}

impl<F: Clone> EM<F> {
    // Maximization step
    fn maximize(&mut self) -> bool {
        // let condition = self.judge_convergence(vec![Matrix::new(0, 0)]);
        unimplemented!()
    }
}

impl<F: Clone> EM<F> {
    fn judge_convergence(&self, new_parameters: Vec<Matrix<F>>) -> bool {
        if self.parameters().len() != new_parameters.len() {
            panic!("cannot calcurate convergence condition because of new parameters size error.")
        }
        unimplemented!()
    }
}

impl<F: Clone> EM<F> {
    pub fn new(mixed_number: usize, variance: F, allowable_error: F, data: Vec<Matrix<F>>) -> Self {
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

impl<F> EM<F> {
    pub fn mixed_number(&self) -> usize {
        self.mixed_number
    }
}

impl<F: Clone> EM<F> {
    pub fn variance(&self) -> F {
        self.variance.clone()
    }
    pub fn parameters(&self) -> Vec<Matrix<F>> {
        self.parameters.clone()
    }
}

#[cfg(test)]
mod test_em {
    use super::*;
}
