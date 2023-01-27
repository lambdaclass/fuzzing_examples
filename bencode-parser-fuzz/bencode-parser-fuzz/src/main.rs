#[macro_use]
extern crate honggfuzz;
use nom_bencode::Value;
use bencoder::bencode::{Bencode, BencodeError};
use std::collections::{BTreeMap, HashMap};


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
    
    match &value[..] {
        [Value::Integer(value), _] => Bencode::BNumber(*value),
        [Value::Bytes(value),_] => to_string_bencode(value),
        [Value::List(value), _] => to_list_bencode(value),
        [Value::Dictionary(value), _] => to_dict_bencode(value),
        [] | [_] | [_, _, _, ..] => panic!("value doesn´t fit a bencode value"),
    }
}

fn value_to_bencode(value: &Value) -> Bencode {
    
    match value {
        Value::Integer(value) => Bencode::BNumber(*value),
        Value::Bytes(value) => to_string_bencode(value),
        Value::List(value) => to_list_bencode(value),
        Value::Dictionary(value) => to_dict_bencode(value),
    }
}

fn to_string_bencode(value: &[u8]) -> Bencode {
    let vector = from_reference_to_vec(value);

    Bencode::BString(vector)
}

fn from_reference_to_vec(value: &[u8]) -> Vec<u8> {
    let vector: Vec<u8> = value.iter().cloned().collect();

    vector.to_vec()
}

fn to_list_bencode(value: &Vec<Value>) -> Bencode {
    let bencode_vec = [];

    for x in value {
        let bencode = value_to_bencode(x);
        bencode_vec.to_vec().push(bencode);
    }

    Bencode::BList(bencode_vec.to_vec())
}

fn to_dict_bencode(value: &HashMap<&[u8], Value>) -> Bencode {
    let btreemap = hashmap_to_btreemap(value);

    Bencode::BDict(btreemap)
}

fn hashmap_to_btreemap(hashmap: &HashMap<&[u8], Value>) -> BTreeMap<Vec<u8>, Bencode> {
    let mut btreemap: BTreeMap<Vec<u8>, Bencode> = BTreeMap::new();

    hashmap
        .into_iter()
        .map(|(key, value)| (from_reference_to_vec(key), value_to_bencode(value)))
        .collect()
}
