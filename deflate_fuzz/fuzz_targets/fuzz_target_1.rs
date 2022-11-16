#![no_main]
use libfuzzer_sys::fuzz_target;
use std::io::prelude::*;
use inflate::inflate_bytes;
use libflate::deflate::Decoder;

// This differential fuzzer panics if two different implementations for deflate decode function returns different results 

fuzz_target!(|data: &[u8]| {
    let mut libflate_decoded = Decoder::new(data);
    let mut decoded_data = Vec::new();
    let libflate_res = libflate_decoded.read_to_end(&mut decoded_data).is_ok();

    let inflate_decoded = inflate_bytes(data).is_ok();

    let _ = match(libflate_res, inflate_decoded){
        (true, true) => true,
        (false, false) => false,
        _ => panic!(
            "differential fuzz failed {}-{}",
            libflate_res, inflate_decoded
        )

    };
});
