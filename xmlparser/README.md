### Xmlparser fuzzer

Fuzzing example with honggfuzz for the xmlparser crate.

From the [xmlparser](https://crates.io/crates/xmlparser) README:
> xmlparser is a low-level, pull-based, zero-allocation XML 1.0 parser.


## How to run

Run hfuzz with the provided dictionary: `cargo hfuzz run xmlparser --dict=xml.dict`
