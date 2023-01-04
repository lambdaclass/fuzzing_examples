#[macro_use]
extern crate honggfuzz;
use nom_bencode::Value;
use bencoder::bencode::{Bencode, BencodeError};


fn main() {
    println!("Starting fuzzer");
    loop {
        fuzz!(|data: &[u8]| {
            let nom_result = parse_source_nom(data).unwrap();
            let libtorrent_result = parse_source_libtorrent(data).unwrap();

            assert_eq!(to_bencode(nom_result), libtorrent_result);
        });
    }
}

fn parse_source_nom(src: &[u8]) -> Result<Vec<nom_bencode::Value>, nom_bencode::Error<&[u8]>> {
    nom_bencode::parse(src)
}

fn parse_source_libtorrent(src: &[u8]) -> Result<Bencode, BencodeError> {
    Bencode::decode(&src.to_vec())
}

fn to_bencode(value: Vec<Value>) -> Bencode {
    match value {
        Value::Integer(value) => Bencode::BNumber(value),
        Value::Bytes(value) => Bencode::BString(value),
        Value::List(value) => Bencode::BList(value),
        Value::Dictionary(value) => Bencode::BDict(value),
    }
}
