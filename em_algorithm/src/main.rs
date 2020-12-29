use em_algorithm::data_manage::*;
use em_algorithm::em::*;

fn main() {
    // read data from mnist_em.csv, which has
    // 21770 handwritten characters' images that consists of 28x28 pixels.
    let data = read_csv::<f64>("data/mnist_em.csv");

    // normalize data
    let data = normalize_data(data, 255f64);

    // define mixed number and variance
    let mixed_number = 3;
    let variance = 0.01;

    // initialize structure for em algorithm
    let mut em = EM::new(mixed_number, variance, data.clone());

    println!("mixed_number: {}, vaiance: {}", em.mixed_number(), em.variance());
    println!("parameters: {:?}", em.parameters());

    let parameters = em.estimate();
    println!("{:?}", parameters);
}
