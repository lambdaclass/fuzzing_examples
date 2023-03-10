### Asn1 fuzzer

Fuzzing example with honggfuzz for the cairo-rs project.

From the [cairo-rs](https://github.com/lambdaclass/cairo-rs) readme:
> A faster and safer implementation of the Cairo VM in Rust


## How to run

Move to the fuzz folder 
`cd fuzz`
Then run honggfuzz with the json dict
`cargo hfuzz run fuzz --dict=json.dict`
