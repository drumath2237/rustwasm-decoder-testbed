use std::io::Cursor;

use zip::ZipArchive;

pub fn zip_test(zip_bytes: &[u8]) -> () {
    let cursor = Cursor::new(zip_bytes);

    let mut archive = ZipArchive::new(cursor).unwrap();

    for i in 0..archive.len() {
        let file = archive.by_index(i).unwrap();
        println!("{}", &file.name())
    }
}
