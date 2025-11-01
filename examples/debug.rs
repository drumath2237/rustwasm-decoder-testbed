use image_webp::WebPDecoder;
use rustwasm_decoder_testbed::test_func;
use std::fs;
use std::io::{Cursor, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut data = Vec::new();
    fs::File::open("./sample_data/pizza/means_l.webp")?.read_to_end(&mut data)?;

    let cursor = Cursor::new(data);
    let mut decoder = WebPDecoder::new(cursor)?;

    let mut pixels = Vec::new();
    decoder.read_image(&mut pixels)?;

    Ok(())
}
