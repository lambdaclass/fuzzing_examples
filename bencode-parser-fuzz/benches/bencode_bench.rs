#[macro_use]
extern crate honggfuzz;

use walkdir::*;
use nom_bencode::Value;
use bencoder::bencode::{Bencode, BencodeError};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::{fs, io};
use rand::Rng;


// fn parse_source(src: &[u8]) -> Result<Vec<nom_bencode::Value>, nom_bencode::Error<&[u8]>> {
//     nom_bencode::parse(src)
// }

fn bencode_benchmark(c: &mut Criterion) {
    let files = fs::read_dir("bencode-parser-fuzz/bencode-parser-fuzz/hfuzz_workspace/bencode-parser-fuzz/input/").unwrap();

    // let paths: Vec<_> = WalkDir::new("bencode-parser-fuzz/bencode-parser-fuzz/hfuzz_workspace/bencode-parser-fuzz/input/")
    //                     .into_iter()
    //                     .filter_map(|f| f.ok()) 
    //                     .map(|f| f.path().to_str().unwrap())
    //                     .collect();

    let mut paths: Vec<&str> = [].to_vec();

    for entry in files {
        let path = entry.unwrap().path();
        // Get path string.
        let path_str = path.to_str().unwrap();
        paths.push(&path_str);
    }
    
    c.bench_function("bencode torrent nom", |b| b.iter(|| parse_source_nom(black_box(paths[0].as_bytes()))));
    c.bench_function("bencode torrent libtorrent", |b| b.iter(|| parse_source_libtorrent(black_box(paths[0].as_bytes()))));
}

criterion_group!(benches, bencode_benchmark);
criterion_main!(benches);


fn parse_source_nom(src: &[u8]) -> Vec<Value>  {
    nom_bencode::parse(src).unwrap()
}

fn parse_source_libtorrent(src: &[u8]) -> Bencode {
    Bencode::decode(&src.to_vec()).unwrap()
}