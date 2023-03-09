### Lambdaworks fuzzer

Fuzzing example for the [lambdaworks](https://github.com/lambdaclass/lambdaworks) project using `cargo-fuzz`.

From the project's readme:
> From the heights of these towers of fields, forty centuries of mathematics look down on us. The library for kids who wanna learn how to do SNARKs and learn other cryptographic stuff too.

## How to run

cd onto the fuzz directory:

`cd fuzz/`

then run any of the targets:

`cargo +nightly fuzz run fuzz_target_x` replace `x` with numbers 1, 2 or 3.
