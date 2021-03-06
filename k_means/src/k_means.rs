use crate::matrix::*;
use num_traits::Float;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct KMeans<F> {
    mixed_number: usize,
    allowable_error: F,
    data: Vec<Matrix<F>>,
    parameters: Vec<Matrix<F>>,
}

impl<F> KMeans<F>
where
    F: Float + FromPrimitive,
{
    pub fn estimate(&mut self) -> (Vec<Matrix<F>>, Vec<(f64, f64)>) {
        let mut count = 1;
        let mut data: Vec<f64> = Vec::new();
        while {
            println!("{} times... ", count);
            count += 1;
            let class = self.classify();
            self.updata_parameters(class, &mut data)
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

impl<F> KMeans<F>
where
    F: Float + FromPrimitive,
{
    fn classify(&self) -> Vec<usize> {
        let mut class = Vec::new();
        for t in 0..self.data.len() {
            let mut norm_v: Vec<(usize, F)> = vec![F::from_f64(0.0).unwrap(); self.mixed_number()]
                .iter()
                .enumerate()
                .map(|e| (e.0, *e.1))
                .collect();
            for i in 0..self.mixed_number() {
                norm_v[i].1 = (&self.data[t] - &self.parameters[i]).norm2();
            }
            norm_v.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            class.push(norm_v[0].0);
        }
        class
    }
}

impl<F> KMeans<F>
where
    F: Float + FromPrimitive,
{
    fn updata_parameters(&mut self, class: Vec<usize>, data: &mut Vec<f64>) -> bool {
        let mut new_parameters: Vec<Matrix<F>> =
            vec![Matrix::new(self.data[0].n(), self.data[0].m()); self.mixed_number()];
        let mut count = vec![0usize; self.mixed_number()];
        for t in 0..self.data.len() {
            new_parameters[class[t]] = &new_parameters[class[t]] + &self.data[t];
            count[class[t]] += 1;
        }
        for i in 0..self.mixed_number() {
            new_parameters[i] = &new_parameters[i] / F::from_usize(count[i]).unwrap();
        }
        let old_parameters = self.parameters();
        self.parameters = new_parameters;

        let mut errors = F::from_f64(0.0).unwrap();
        for i in 0..self.mixed_number() {
            let e = (&old_parameters[i] - &self.parameters()[i]).norm2::<F>();
            errors = errors + e;
            println!("  {}'s error: {}", i, e.to_f64().unwrap());
        }
        data.push(errors.to_f64().unwrap());
        println!("      errors: {}", errors.to_f64().unwrap());
        errors > self.allowable_error()
    }
}

impl<F: Clone> KMeans<F> {
    pub fn new(mixed_number: usize, allowable_error: F, data: Vec<Matrix<F>>) -> Self {
        let mut rng = rand::thread_rng();

        let mut parameters = Vec::new();
        for _ in 0..mixed_number {
            let index = rng.gen::<usize>() % data.len();
            parameters.push((*data.index(index)).clone());
        }

        KMeans {
            mixed_number,
            allowable_error,
            data,
            parameters,
        }
    }
}

impl<F> KMeans<F> {
    pub fn mixed_number(&self) -> usize {
        self.mixed_number
    }
}

impl<F: Clone> KMeans<F> {
    pub fn allowable_error(&self) -> F {
        self.allowable_error.clone()
    }
    pub fn parameters(&self) -> Vec<Matrix<F>> {
        self.parameters.clone()
    }
}
