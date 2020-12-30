use k_means::data_manage::*;
use k_means::k_means::*;
use k_means::draw::*;

fn main() {
    // read data
    let data = read_csv::<f64>("data/mnist_em.csv");

    // normalize data
    let data = normalize_data(data, 255f64);

    // split data for learing and for test
    let (training_data, test_data) = split_data_at(21700, data);

    // define mixed number and allowable error
    let mixed_number = 3;
    let allowable_error = 19.8;

    let mut k_means = KMeans::new(mixed_number, allowable_error, training_data);

    let parameters = k_means.estimate();

    for i in 0..k_means.mixed_number() {
        let _ = write_image(&format!("images/image{}of{}.png", i, k_means.mixed_number()), parameters[i].clone());
    }
    println!("{:?}", parameters);
}
