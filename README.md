
Fuzzing Examples
===


Description
---

This is a compilation of fuzzers written in Rust that use either Honggfuzz or Cargofuzz as a tool.

Depending on which tool is being used the fuzzer runs differently.

To see the usage of this libraries check their documentation:
 * [honggfuzz](https://github.com/google/honggfuzz)
 * [cargofuzz](https://github.com/rust-fuzz/cargo-fuzz)

Fuzzers structure 
---

In this repo the fuzzers have the following structure:

### Honggfuzz' fuzzer

* The  ***src*** folder contains the file main with the fuzzer 
* The ***hfuzz_workspace*** folder contains a REPORT file with the report of the crashes the fuzzer found, this folder is updated each time the fuzzer finds a new crash and has the information about the crash along with the name of the input that triggered it. Also contains the input files that trigger the errors, the file names start with SIGABRT

### Cargofuzz' fuzzer

* the ***fuzz_targets*** contains the files with the fuzzers

## User Guide

### 1. Create the docker container 
To create and run the docker container ready to run the fuzzers use the command:
`make docker-up`


### 2. Run a fuzzer 

Each fuzzer has its own Readme with specific instructions to run each of them.

To run a fuzzer with default settings, step into the fuzzer folder and use the command:

For cargo-fuzz fuzzers:
`make run-libfuzzer <fuzzer name>`

For honggfuzz fuzzers:
`make run-honggfuzzer <fuzzer name>`
