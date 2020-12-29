use crate::matrix::*;
use num_traits::Float;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct KMeans<F> {
    mixed_number: usize,
    data: Vec<Matrix<F>>,
    parameters: Vec<Matrix<F>>
}

impl<F> KMeans<F> 
where
    F: Float
{
    pub fn estimate(&mut self) -> Vec<Matrix<F>> {
        let mut count = 1;
        while {
            println!("{} times... now loading...", count);
            count += 1;
            let class = self.classify();
            self.updata_parameters(class)
        } {}
        self.parameters()
    }
}

impl<F> KMeans<F> 
where
    F: Float + FromPrimitive
{
    fn classify(&self) -> Vec<usize> {
        let mut class = Vec::new();
        for t in 0..self.data.len() {
            let norm_v: Vec<(usize, F)> = vec![F::from_f64(0.0).unwrap(); self.mixed_number()]
                .iter()
                .enumerate()
                .map(|e| (e.0, *e.1))
                .collect();
            for i in 0..self.mixed_number() {
                //
            }
        }

        unimplemented!()
    }
}

impl<F> KMeans<F> {
    fn updata_parameters(&mut self, class: Vec<usize>) -> bool {
        unimplemented!()
    }
}

impl<F: Clone> KMeans<F> {
    pub fn new(mixed_number: usize, data: Vec<Matrix<F>>) -> Self {
        let mut rng = rand::thread_rng();

        let mut parameters = Vec::new();
        for _ in 0..mixed_number {
            let index = rng.gen::<usize>() % data.len();
            parameters.push((*data.index(index)).clone());
        }

        KMeans {
            mixed_number,
            data,
            parameters
        }
    }
}

impl<F> KMeans<F> {
    pub fn mixed_number(&self) -> usize {
        self.mixed_number
    }
}

impl<F: Clone> KMeans<F> {
    pub fn parameters(&self) -> Vec<Matrix<F>> {
        self.parameters.clone()
    }
}