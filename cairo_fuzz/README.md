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
For this fuzzer to work you need to install `cairo-lang`, some of it's dependencies also need to have gmp installed:
```sh
sudo apt install -y libgmp3-dev
python3.9 -m venv ~/cairo_venv
source ~/cairo_venv/bin/activate
pip3 install cairo-lang
```
This should work assuming you're using the Docker image or running a Debian based linux distro.

Once the installation is complete, you can run the following commands to run the fuzzer

`cd diff-fuzz`
`make` optional, but it's recommended since it builds a corpus
`HFUZZ_RUN_ARGS="--dict=json.dict --input=corpus" cargo hfuzz run diff-fuzz` remember to omit `--input=corpus` if you didn't run the previous step.
