### Asn1 fuzzer

Fuzzing example with honggfuzz for the cairo-rs project.

From the [cairo-rs](https://github.com/lambdaclass/cairo-rs) README:
> A faster and safer implementation of the Cairo VM in Rust


## How to run

# Regular fuzzer
Move to the fuzz folder 
`cd fuzz`
Then run honggfuzz with the json dict
`HFUZZ_RUN_ARGS="--dict=json.dict" cargo hfuzz run fuzz`

# Differential fuzzer
`cd diff-fuzz`
`HFUZZ_RUN_ARGS="--input=cairo_programs" cargo hfuzz run diff-fuzz`
