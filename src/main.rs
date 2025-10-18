use std::{fs::File, io::Read};

mod zip_test;

fn main() {
    let mut file = File::open("./sample_data/pizza.sog").unwrap();
    let mut bytes = Vec::new();
    let _size = file.read_to_end(&mut bytes).unwrap();
    zip_test::zip_test(&bytes)
}
