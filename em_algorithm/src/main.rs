use em_algorithm::data_manage::*;
use em_algorithm::em::*;

fn main() {
    // read data from mnist_em.csv, which has
    // 21770 handwritten characters' images that consists of 28x28 pixels.
    let data = read_csv::<f64>("data/mnist_em.csv");

    // normalize data
    let data = normalize_data(data, 255f64);

    let em = EM::new(data.clone());
}
