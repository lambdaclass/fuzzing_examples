#[macro_use]
extern crate honggfuzz;
extern crate tiff;

// to run this fuzzer use honggfuzz to run image-tiff
use std::io::Cursor;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let mut file = Cursor::new(data);
            if let Ok(mut decoder) = tiff::decoder::Decoder::new(file){
                let _ = decoder.read_image();
            }    
        });
    }
}

