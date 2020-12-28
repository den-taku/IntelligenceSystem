use std::fs::File;

fn main() {
    let mut file = File::open("mnist_em.csv").expect("fail to open mnist_em.csv");
    let mut file_for_input = File::create("src/input.rs").expect("fail to open input.rs");
    println!("Hello, world!");
}
