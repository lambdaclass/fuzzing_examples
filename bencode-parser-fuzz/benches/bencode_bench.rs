#[macro_use]
extern crate honggfuzz;

use walkdir::*;
use nom_bencode::Value;
use bencoder::bencode::{Bencode, BencodeError};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::{fs, io};

static SOURCE_BYTES: &'static [u8] = include_bytes!("../../torrents/file1.torrent");

fn bencode_benchmark(c: &mut Criterion) {

    // commented for following implementation
    // let paths: Vec<_> = WalkDir::new("bencode-parser-fuzz/bencode-parser-fuzz/hfuzz_workspace/bencode-parser-fuzz/input/")
    //                     .into_iter()
    //                     .filter_map(|f| f.ok()) 
    //                     .map(|f| f.path().to_str().unwrap())
    //                     .collect();

    // let mut paths: Vec<&str> = [].to_vec();

    // for entry in files {
    //     let path = entry.unwrap().path();
    //     // Get path string.
    //     let path_str = path.to_str().unwrap();
    //     paths.push(&path_str);
    // }
    
    c.bench_function("bencode torrent nom", |b| b.iter(|| parse_source_nom(black_box(SOURCE_BYTES))));
    c.bench_function("bencode torrent libtorrent", |b| b.iter(|| parse_source_libtorrent(black_box(SOURCE_BYTES))));
}

criterion_group!(benches, bencode_benchmark);
criterion_main!(benches);


fn parse_source_nom(src: &[u8]) -> Vec<Value>  {
    nom_bencode::parse(src).unwrap()
}

fn parse_source_libtorrent(src: &[u8]) -> Bencode {
    Bencode::decode(&src.to_vec()).unwrap()
}