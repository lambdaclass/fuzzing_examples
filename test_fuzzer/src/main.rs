use honggfuzz::fuzz;

// This fuzzer is used for testing purposes
fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if data.len() > 20 {
                panic!("bigger than 20")
            } 
        });
    }
}
