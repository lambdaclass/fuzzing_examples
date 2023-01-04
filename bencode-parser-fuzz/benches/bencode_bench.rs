#[macro_use]
extern crate honggfuzz;
use nom_bencode::Value;
use bencoder::bencode::{Bencode, BencodeError};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn parse_source(src: &[u8]) -> Result<Vec<nom_bencode::Value>, nom_bencode::Error<&[u8]>> {
    nom_bencode::parse(src)
}

fn bencode_benchmark(c: &mut Criterion) {
    c.bench_function("bencode torrent nom", |b| b.iter(|| parse_source_nom(black_box(include_bytes!(data.to_string())))));
    c.bench_function("bencode torrent libtorrent", |b| b.iter(|| parse_source_libtorrent(black_box(include_bytes!(data.to_string())))));
}

criterion_group!(benches, bencode_benchmark);
criterion_main!(benches);


fn parse_source_nom(src: &[u8]) -> Result<Vec<nom_bencode::Value>, nom_bencode::Error<&[u8]>> {
    nom_bencode::parse(src)
}

fn parse_source_libtorrent(src: &[u8]) -> Result<Bencode, BencodeError> {
    Bencode::decode(&src.to_vec())
}