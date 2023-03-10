### Deflate fuzzer

Differential fuzzing example for the crates: [libflate](https://crates.io/crates/libflate) and [inflate](https://crates.io/crates/inflate) using `cargo-fuzz`. Both libraries decode data compressed with the [DEFLATE](https://en.wikipedia.org/wiki/Deflate) algorithm, libflate also allows for encoding.

## How to run

Run with the command `cargo +nightly fuzz run fuzz_target_1`
