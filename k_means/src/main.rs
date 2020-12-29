use k_means::data_manage::*;
use k_means::k_means::*;

fn main() {
    // read data
    let data = read_csv::<f64>("data/mnist_em.csv");

    // normalize data
    let data = normalize_data(data, 255f64);

    // split data for learing and for test
    let (training_data, test_data) = split_data_at(21700, data);

    // define mixed number
    let mixed_number = 3;

    let mut k_means = KMeans::new(mixed_number, training_data);

    let parameters = k_means.estimate();
    println!("{:?}", parameters);
}
