use crate::matrix::*;
use num_traits::Float;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct EM<F> {
    mixed_number: usize,
    variance: F,
    allowable_error: F,
    data: Vec<Matrix<F>>,
    parameters: Vec<Matrix<F>>,
}

impl<F> EM<F>
where
    F: Clone + Float + FromPrimitive,
{
    pub fn estimate(&mut self) -> (Vec<Matrix<F>>, Vec<(f64, f64)>) {
        let mut count = 1;
        let mut data: Vec<f64> = Vec::new();
        while {
            println!("{} times...", count);
            count += 1;
            let (one_i, x_i) = self.expect();
            self.maximize(one_i, x_i, &mut data)
        } {}
        (
            self.parameters(),
            data.iter()
                .enumerate()
                .map(|e| ((e.0).to_f64().unwrap(), *e.1))
                .collect(),
        )
    }
}

impl<F> EM<F>
where
    F: Float + FromPrimitive + ToPrimitive,
{
    // Expectation step
    fn expect(&mut self) -> (Vec<F>, Vec<Matrix<F>>) {
        // 1.calcurate posterior probability

        // for numerator
        let mut p_vec = Vec::new();
        // for denominator
        let mut sigma_p = vec![F::from_f64(0.0).unwrap(); self.data.len()];

        // calcurate numerators and addassign to denominator
        for t in 0..self.data.len() {
            for i in 0..self.mixed_number() {
                let p = self.calcurate_p(t, i);
                p_vec.push(p);
                sigma_p[t] = sigma_p[t] + p;
            }
        }

        // divide numerator by denominator
        for t in 0..self.data.len() {
            for i in 0..self.mixed_number() {
                p_vec[self.mixed_number() * t + i] =
                    p_vec[self.mixed_number() * t + i] / sigma_p[t];
            }
        }

        // 2.calcurate sufficient statistics

        let mut one_i = vec![F::from_f64(0.0).unwrap(); self.mixed_number()];
        let mut x_i =
            vec![Matrix::<F>::new(self.data[0].n(), self.data[0].m()); self.mixed_number()];

        // sigma
        for t in 0..self.data.len() {
            for i in 0..self.mixed_number() {
                one_i[i] = one_i[i] + p_vec[self.mixed_number() * t + i];
                x_i[i] = &x_i[i] + &(&self.data[t] * p_vec[self.mixed_number() * t + i]);
            }
        }

        // 1/T
        for i in 0..self.mixed_number() {
            one_i[i] = one_i[i] / F::from_f64((self.data.len()).to_f64().unwrap()).unwrap();
            x_i[i] = &x_i[i] / F::from_f64((self.data.len()).to_f64().unwrap()).unwrap();
        }

        (one_i, x_i)
    }

    // calcurate f(t, i) = exp(-1/2σ^2 |x(t) - μi|^2)
    fn calcurate_p(&self, t: usize, i: usize) -> F {
        let x: f64 = -((&self.data[t] - &self.parameters()[i]).norm2_row::<f64>()
            / 2.0f64
            / (self.variance()).to_f64().unwrap()
            / (self.variance()).to_f64().unwrap());
        F::from_f64((x).exp()).unwrap()
    }
}

impl<F> EM<F>
where
    F: Float + FromPrimitive,
{
    // Maximization step
    fn maximize(&mut self, one_i: Vec<F>, x_i: Vec<Matrix<F>>, mut data: &mut Vec<f64>) -> bool {
        let past_parameters = self.parameters();
        let mut new_parameters = Vec::new();

        // update parameters
        for i in 0..self.mixed_number() {
            new_parameters.push(&x_i[i] / one_i[i].clone());
        }

        self.parameters = new_parameters;

        self.judge_convergence(past_parameters, &mut data)
    }
}

impl<F> EM<F>
where
    F: Float + FromPrimitive,
{
    fn judge_convergence(&self, past_parameters: Vec<Matrix<F>>, data: &mut Vec<f64>) -> bool {
        // check parameter's size
        if self.parameters().len() != past_parameters.len() {
            panic!("cannot calcurate convergence condition because of new parameters size error.")
        }

        // calcurate error
        let mut error = F::from_f64(0.0).unwrap();
        for i in 0..self.mixed_number() {
            error = error + (&self.parameters()[i] - &past_parameters[i]).norm2();
        }

        println!("      error size is {}", error.to_f64().unwrap());
        data.push(error.to_f64().unwrap());

        // check condition
        !(error < self.allowable_error)
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
            parameters,
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
