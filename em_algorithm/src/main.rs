use em_algorithm::data_manage::*;
use em_algorithm::draw::*;
use em_algorithm::em::*;

fn main() {
    // read data from mnist_em.csv, which has
    // 21770 handwritten characters' images that consists of 28x28 pixels.
    let data = read_csv::<f64>("data/mnist_em.csv");

    // normalize data
    let data = normalize_data(data, 255f64);

    // split data for learing and for test
    let (training_data, _test_data) = split_data_at(21770, data);

    // define mixed number, variance, and allowable error
    let mixed_number = 3;
    let variance = 1.0;
    let allowable_error = 0.01;

    // initialize structure for em algorithm
    let mut em = EM::new(
        mixed_number,
        variance,
        allowable_error,
        training_data.clone(),
    );

    // use em algorithm
    let parameters = em.estimate();

    for i in 0..em.mixed_number() {
        let _ = write_image(&format!("images/image{}of{}.png", i, em.mixed_number()), parameters[i].clone());
    }
    println!("{:?}", parameters);

    // TODO: test
    // test_data;
    // parameters;
}
