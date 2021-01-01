use k_means::data_manage::*;
use k_means::draw::*;
use k_means::k_means::*;
use num_traits::ToPrimitive;
use quanta::Clock;

fn main() {
    // for measuring time
    let mut clock = Clock::new();
    let start = clock.now();

    // read data
    let data = read_csv::<f64>("data/mnist_em.csv");

    // normalize data
    let data = normalize_data(data, 255f64);

    // split data for learing and for test
    let (training_data, _test_data) = split_data_at(21700, data);

    // define mixed number and allowable error
    let mixed_number = 3;
    let allowable_error = 0.01;

    let mut k_means = KMeans::new(mixed_number, allowable_error, training_data);

    let (parameters, data) = k_means.estimate();

    for i in 0..k_means.mixed_number() {
        let _ = write_image(
            &format!("images/image{}of{}.png", i, k_means.mixed_number()),
            parameters[i].clone(),
        );
    }
    // println!("{:?}", parameters);

    // show time
    let stop = clock.now();
    println!("need {:?}.", stop.duration_since(start));

    // draw semi-log graph with gnuplot
    draw_graph_log10(
        0.0,
        (data.len() + 1).to_f64().unwrap(),
        1e-11,
        10.0,
        "times",
        "errors",
        "blue",
        data,
    );
}
