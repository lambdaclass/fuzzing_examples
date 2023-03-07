
Fuzzing Examples
===

## Table of Contents

[TOC]


Tools
---

This is a compilation of fuzzers written in Rust that use either Honggfuzz or Cargofuzz as a tool.

Depending on which tool is being used the fuzzer runs differently.

To see the usage of this libraries check their documentation:
 * [honggfuzz](https://github.com/google/honggfuzz)
 * [cargofuzz](https://github.com/rust-fuzz/cargo-fuzz)

## User Guide

Depending on the fuzzer use the following commands:

### If the Fuzzer Uses Honggfuzz

* to run it with total random data use: 
`cargo hfuzz run <fuzzer name>`
* to run it with an input dictionary use: 
`HFUZZ_RUN_ARGS=--dict=dictionaryfle.dict cargo hfuzz run <fuzzer name>`
* to run it with an initial collection of inputs use: 
`cargo hfuzz run <fuzzer name> -i <path to inputs folder>`

* to debug with certain input use: 
`cargo hfuzz run-debug <fuzzer name> <input to debug the fuzzer>`

### if the fuzzer uses cargofuzz

* to run it with random data use:
`cargo +nightly fuzz run <target file name>`


Fuzzers structure 
---

In this repo the fuzzers have the following structure:

### if the fuzzer uses honggfuzz 

* The  ***src*** folder contains the file main with the fuzzer 
* The ***hfuzz_workspace*** folder contains a REPORT file with the report of the crashes the fuzzer found, this folder is updated each time the fuzzer finds a new crash and has the information about the crash along with the name of the input that triggered it. Also contains the input files that triggers the errors, the file names starts with SIGABRT

### if the fuzzer uses cargofuzz 

* the ***fuzz_targets*** contains the files with the fuzzers
