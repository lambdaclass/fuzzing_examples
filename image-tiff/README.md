### Tiff fuzzer

Fuzzing example with honggfuzz for the tiff crate.

From the [tiff](https://crates.io/crates/tiff) README:
> TIFF decoding and encoding library in pure Rust


## How to run

Run hfuzz with the provided dictionary: `cargo hfuzz run image-tiff --dict=tiff.dict`
