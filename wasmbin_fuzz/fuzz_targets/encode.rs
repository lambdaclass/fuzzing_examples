#![no_main]
//to run this fuzzer you need the wasmbin repository, add this folder as fuzz in the repository and run the fuzzer with cargo fuzz and if you want you can add a corpus from https://github.com/WAVM/wavm-fuzz-corpora
use libfuzzer_sys::fuzz_target;

use wasmbin::Module;

fuzz_target!(|data: &[u8]| {
    if let Ok(decoded) = Module::decode_from(data) {
        let _ = decoded.encode_into(Vec::new());
    }
});
