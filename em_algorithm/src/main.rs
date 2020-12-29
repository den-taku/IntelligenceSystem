use em_algorithm::data_manage::*;
use em_algorithm::em::*;

fn main() {
    // read data from mnist_em.csv, which has
    // 21770 handwritten characters' images that consists of 28x28 pixels.
    let data = read_csv::<f64>("data/mnist_em.csv");

    // normalize data
    let data = normalize_data(data, 255f64);

    // split data for learing and for test
    let (training_data, test_data) = split_data_at(21760, data);

    // define mixed number, variance, and allowable error
    let mixed_number = 3;
    let variance = 0.01;
    let allowable_error = 0.1e-9;

    // initialize structure for em algorithm
    let mut em = EM::new(mixed_number, variance, allowable_error ,training_data.clone());

    // use em algorithm
    let parameters = em.estimate();

    // test 
    test_data;
    parameters;
}
