
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

### 1. create the docker container 
* Use the command  `docker build -t ready-to-fuzz-image .` inside this project to create a docker image named **ready-to-fuzz-image** from the dockerfile in the project.

* run the command `docker run -it --rm --cap-add=SYS_PTRACE --security-opt seccomp=unconfined ready-to-fuzz-image bash` to run the container from the image previously created.

### 2. Run a fuzzer 

Depending on the fuzzer use the following commands:

### If the Fuzzer Uses Honggfuzz

* to run it with totally random data use: 
`cargo hfuzz run <fuzzer name>`
* to run it with an input dictionary use: 
`HFUZZ_RUN_ARGS=--dict=<dictionaryfile>.dict cargo hfuzz run <fuzzer name>`
* to run it with an initial collection of inputs use: 
`cargo hfuzz run <fuzzer name> -i <path to inputs folder>`

* to debug with certain input use: 
`cargo hfuzz run-debug <fuzzer name> <input to debug the fuzzer>`

### if the fuzzer uses cargofuzz

* to run it with random data use:
`cargo +nightly fuzz run <target file name>`
