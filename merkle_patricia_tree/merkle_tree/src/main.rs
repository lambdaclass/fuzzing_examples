#[macro_use]
extern crate honggfuzz;
use cita_trie::MemoryDB;
use cita_trie::{PatriciaTrie, Trie};
use hasher::HasherKeccak;
use patricia_merkle_tree::PatriciaMerkleTree;
use sha3::Keccak256;
use proptest::prelude::*;
use std::sync::Arc;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {

            let mut tree = PatriciaMerkleTree::<&[u8], &[u8], Keccak256>::new();

            tree.insert(data, data);
            
            let root_hash = tree.compute_hash().as_slice().to_vec();
        
            let memdb = Arc::new(MemoryDB::new(true));
            let hasher = Arc::new(HasherKeccak::new());
            let mut trie = PatriciaTrie::new(Arc::clone(&memdb), Arc::clone(&hasher));
  
            trie.insert(data.to_vec(), data.to_vec()).unwrap();
            
            let reference_root = trie.root().unwrap();
        
            assert_eq!(
            reference_root,
            root_hash
            )
        });
    }
}


