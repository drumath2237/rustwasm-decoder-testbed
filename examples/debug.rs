use std::fs;
use std::io::Read;

use rustwasm_decoder_testbed::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut data = Vec::new();
    fs::File::open("./sample_data/pizza/means_l.webp")?.read_to_end(&mut data)?;

    let pixels = decode_webp(&data);

    Ok(())
}
