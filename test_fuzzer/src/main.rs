use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if data.len() > 20 {
                panic!("bigger than 20")
            } 
        });
    }
}
