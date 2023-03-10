### Wasmbin fuzzer

Fuzzing example with `cargo-fuzz` for the [wasmbin](https://crates.io/crates/wasmbin) crate

From the library's readme:
> wasmbin is a library implementing parsing and serialization WebAssembly binaries.

## How to run

Run any of the targets with: `cargo +nightly fuzz run decode`
The available targets are `decode` and `encode`
