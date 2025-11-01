use image_webp::WebPDecoder;
use std::fs;
use std::io::{Cursor, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut data = Vec::new();
    fs::File::open("./sample_data/pizza/means_l.webp")?.read_to_end(&mut data)?;

    let cursor = Cursor::new(data);
    let mut decoder = WebPDecoder::new(cursor)?;

    let output_size = decoder.output_buffer_size().unwrap();
    let mut pixels = vec![0; output_size];
    decoder.read_image(&mut pixels)?;

    Ok(())
}
