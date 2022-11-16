#[macro_use]
extern crate honggfuzz;
extern crate xmlparser;

fn main() {
    loop{
        fuzz!(|data: &[u8]| {

            let data = match std::str::from_utf8(data) {
                Ok(o) => o,
                Err(_) => return,
            };

            for token in xmlparser::Tokenizer::from(data) {
                println!("{:?}", token);
            };
        });
    }
}
