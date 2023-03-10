### Bencode parser fuzzer

Benchmark comparison fuzzer for [libtorrent-rs](https://github.com/lambdaclass/libtorrent-rs) vs [nom-bencode](https://github.com/edg-l/nom-bencode) using honggfuzz.

## How to run

cd into bencode-parser-fuzz:
`cd bencode-parser-fuzz`
run the fuzzer:
`cargo hfuzz run bencode-parser-fuzz`

