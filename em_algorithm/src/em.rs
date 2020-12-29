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
            let (one_i, x_i) = self.expect();
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
    fn expect(&mut self) -> (Vec<F>, Vec<Matrix<F>>) {
        // 1.calcurate posterior probability

        // for numerator
        let mut p_vec = Vec::new();
        // for denominator
        let mut sigma_p = vec![F::from_f64(0.0).unwrap(); self.mixed_number()];

        // calcurate numerators and addassign to denominator 
        for t in 0..self.data.len() {
            for i in 0..self.mixed_number() {
                let p = self.calcurate_p(t, i);
                p_vec.push(p);
                sigma_p[i] = sigma_p[i] + p;
            }
        }

        // divide numerator by denominator
        for t in 0..self.data.len() {
            for i in 0..self.mixed_number() {
                p_vec[3 * t + i] = p_vec[3 * t + i] / sigma_p[i];
            }
        }

        // 2.calcurate sufficient statistics

        let mut one_i = vec![F::from_f64(0.0).unwrap(); self.mixed_number()];
        let mut x_i = vec![Matrix::<F>::new(self.data[0].n(), self.data[0].m())];

        // sigma
        for t in 0..self.data.len() {
            for i in 0..self.mixed_number() {
                one_i[i] = one_i[i] + p_vec[3 * t + i];
                x_i[i] = &x_i[i] + &(&self.data[t] + p_vec[3 * t + i]);
            }
        }

        // 1/T
        for i in 0..self.mixed_number() {
            one_i[i] = one_i[i] / F::from_f64((self.data.len()).to_f64().unwrap()).unwrap();
            x_i[i] = &x_i[i] / F::from_f64((self.data.len()).to_f64().unwrap()).unwrap(); 
        }
        
        (one_i, x_i)
    }

    fn calcurate_p(&self, t: usize, i: usize) -> F {
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