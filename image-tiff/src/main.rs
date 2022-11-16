#[macro_use]
extern crate honggfuzz;
extern crate tiff;

use std::io::Cursor;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let mut file = Cursor::new(Vec::new());
            if let Ok(mut decoder) = tiff::decoder::Decoder::new(file){
                let _ = decoder.read_image();
            }    
        });
    }
}

