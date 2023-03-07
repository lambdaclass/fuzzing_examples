#[macro_use]
extern crate honggfuzz;
use cita_trie::MemoryDB;
use cita_trie::{PatriciaTrie, Trie};
use hasher::HasherKeccak;
use patricia_merkle_tree::PatriciaMerkleTree;
use sha3::Keccak256;
use proptest::prelude::*;
use std::sync::Arc;
use rand::{rngs::StdRng, Rng, SeedableRng};

const MAX_INPUT_SIZE: usize = 1024;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {

            let mut tree = PatriciaMerkleTree::<&[u8], &[u8], Keccak256>::new();
        
            let memdb = Arc::new(MemoryDB::new(true));
            let hasher = Arc::new(HasherKeccak::new());
            let mut trie = PatriciaTrie::new(Arc::clone(&memdb), Arc::clone(&hasher));
            let mut init_input: Vec<u8> = data.to_vec();
    
            let seed = if data.len() > 3 {
                let mut dst = [0u8; 4];
                dst.clone_from_slice(&data[0..4]);
                u64::from(u32::from_be_bytes(dst))
            }else{
                1234567
            };
            
            let inputs = mutate(&mut init_input, seed);

            for input in inputs.iter() {
                trie.insert(input.to_vec(), input.to_vec());

                tree.insert(&input, &input);

                let root_hash = tree.compute_hash().as_slice().to_vec();
                let reference_root = trie.root().unwrap();
            
                assert_eq!(
                reference_root,
                root_hash
                )
            };
            
        });
    }
}

fn mutate(input:  &mut [u8], seed: u64) -> Vec<Vec<u8>> {
    /// List of mutation strategies which do not require an input database
    const STRATEGIES: &[for<'a> fn(&'a mut [u8], u64) -> Vec<u8>; 9] = &[
        shrink,
        expand,
        add_sub,
        set,
        swap,
        copy,
        inter_splice,
        insert_rand,
        overwrite_rand,
        // Mutator::byte_repeat_overwrite,
        // Mutator::byte_repeat_insert,
        // Mutator::magic_overwrite,
        // Mutator::magic_insert,
        // Mutator::random_overwrite,
        // Mutator::random_insert,
        // Mutator::bit,
        // Mutator::inc_byte,
        // Mutator::dec_byte,
        // Mutator::neg_byte,
    ];

    let mut vec = vec![];

    vec.push(input.to_vec());

    let mut rng = StdRng::seed_from_u64(1234567);

    for _ in 0..6 {
        // Pick a random mutation strategy
        let sel = rand_number_in_range(0, STRATEGIES.len() - 1, seed);
            
        // Get the strategy
        let strat = STRATEGIES[sel];

        // Run the mutation strategy
        let new_input = strat(input, seed);

        // Add new mutation to vec
        vec.push(new_input);
    };
     
    vec
}

fn rand_offset(input: &[u8], seed: u64 ) -> usize {

    let offset_range = input.len() - 1 as usize;

    if offset_range > 1{
        // just return a random index
        rand_number_in_range(0,offset_range, seed)
    } else {
        // Input is entirely empty, just return index 0 such that
        // things that insert into the input know that they should
        // just insert at 0.
        0
    }
}

// generates a random number within a range
fn rand_number_in_range(min: usize, max: usize, seed: u64) -> usize {
    let mut rng = StdRng::seed_from_u64(seed);
    
    if min == max {
        // if there´s no range, return the same number
        min
    } else { 
        let random = rng.gen_range(min..max);
        random
    }
}

/// Randomly delete a chunk of the input
fn shrink(input:  &mut [u8], seed: u64) -> Vec<u8> {
    if input.is_empty() {
        return input.to_vec();
    }
  
    // Pick a random offset to remove data at
    let offset = rand_offset(input, seed);
    // Compute the number of bytes we could remove from this offset
    let can_remove = input.len() - offset;

    // Compute a maximum number of bytes to remove
    let max_remove = if rand_number_in_range(0,15, seed) != 0 {
        // 14 in 15 chance of removing at most 16 bytes, this limits the
        // amount we remove in the most common case
        core::cmp::min(16, can_remove)
    } else {
        // 1 in 15 chance of removing a random amount of bytes to the end
        // of the input
        can_remove
    };

    // Pick the amount of bytes to remove
    let to_remove = rand_number_in_range(1,max_remove, seed);

    // Remove the bytes from the input
    let mut binding = input.to_vec();

    let new_input = binding.drain(offset..offset + to_remove).as_slice().to_vec();

    new_input
}

/// Make space in the input, filling with zeros
fn expand(input: &mut [u8], seed: u64) -> Vec<u8> {
    // Pick a random offset to expand at
    let offset = rand_offset(input, seed);

    // Compute the number of bytes we could expand from this offset
    let can_expand = input.len() + offset;

    // Compute a maximum number of expansion bytes
    let max_expand = if rand_number_in_range(0,15, seed) != 0 {
        // 15 in 16 chance of capping expansion to 16 bytes
        core::cmp::min(16, can_expand)
    } else {
        // 1 in 16 chance of uncapped expansion
        can_expand
    };

    // Create what to expand with
    let iter = core::iter::repeat(b'\0').take(rand_number_in_range(1,max_expand, seed)); 

    let mut new_input = input.to_owned();
    
    // Expand at `offset` with `iter`
    new_input.to_vec().splice(offset..offset, iter);

    new_input.to_vec()
}

/// Add or subtract a random amount with a random endianness from a random
/// size `u8` through `u64`
fn add_sub(input: &mut [u8], seed: u64) -> Vec<u8> {
    // Nothing to do on an empty input
    if input.is_empty() {
        return input.to_vec();
    }

    // Pick an offset to corrupt at
    let offset = rand_offset(input, seed);

    // Get the remaining number of bytes in the input
    let remain = input.len() - offset;

    // Pick a random size of the add or subtract as a 1, 2, 4, or 8 byte
    // signed integer
    let intsize = match remain {
        1..=1                => 1,
        2..=3                => 1 << rand_number_in_range(0,2, seed),
        4..=7                => 1 << rand_number_in_range(0,3, seed),
        8..=core::usize::MAX => 1 << rand_number_in_range(0,4, seed),
        _ => unreachable!(),
    };

    // Determine the maximum number to add or subtract
    let range = match intsize {
        1 => 16,
        2 => 4096,
        4 => 1024 * 1024,
        8 => 256 * 1024 * 1024,
        _ => unreachable!(),
    };

    // Convert the range to a random number from [-range, range]
    let delta = rand_number_in_range(0,range * 2, seed) as i32 - range as i32;

    let mut new_input = input.to_owned();

    /// Macro to mutate bytes in the input as a `$ty`
    macro_rules! mutate {
        ($ty:ty) => {{
            // Interpret the `offset` as a `$ty`
            let tmp = <$ty>::from_ne_bytes(
                new_input[offset..offset + intsize].try_into().unwrap());

            // Apply the delta, interpreting the bytes as a random
            // endianness
            let tmp = if rand_number_in_range(0,1, seed) == 0 {
                tmp.wrapping_add(delta as $ty)
            } else {
                tmp.swap_bytes().wrapping_add(delta as $ty).swap_bytes()
            };

            // Write the new value out to the input
            new_input[offset..offset + intsize].copy_from_slice(
                &tmp.to_ne_bytes());
            new_input.to_vec()
        }}
    }

    // Apply the delta to the offset
    match intsize {
        1 => mutate!(u8),
        2 => mutate!(u16),
        4 => mutate!(u32),
        8 => mutate!(u64),
        _ => unreachable!(),
    }

}

/// Randomly replace a sequence of bytes with the same random character
/// repeated a random amount of times
fn set(input: &mut [u8], seed: u64) -> Vec<u8> {
    // Nothing to do on an empty input
    if input.is_empty() {
        return input.to_vec();
    }

    // Pick offset to memset at
    let offset = rand_offset(input, seed);

    // Pick random length to remainder of input
    let len = rand_number_in_range(1,input.len() - offset, seed);

    // Pick the value to memset
    let chr = rand_number_in_range(0,255, seed) as u8;

    let mut new_input = input.to_owned();
   
    // Replace the selected bytes at the offset with `chr`
    new_input[offset..offset + len].iter_mut().for_each(|x| *x = chr);

    new_input.to_vec()
}

/// Swap two difference sequence of bytes in the input to different places
fn swap(input: &mut [u8], seed: u64) -> Vec<u8> {
    // Nothing to do on an empty input
    if input.is_empty() {
        return input.to_vec();
    }

    // Pick two random ranges in the input and calculate the remaining
    // bytes for them
    let src    = rand_offset(input, seed);
    let srcrem = input.len() - src;
    let dst    = rand_offset(input, seed);
    let dstrem = input.len() - dst;

    // Pick a random length up to the max for both offsets
    let len = rand_number_in_range(1,core::cmp::min(srcrem, dstrem), seed);

    // Swap the ranges of bytes
    swap_ranges(input, src, dst, len)
}

/// Swap two ranges in an input buffer
fn swap_ranges(input: &mut [u8], mut offset1: usize, mut offset2: usize,
    mut len: usize) -> Vec<u8> {
    if offset1 < offset2 && offset1 + len >= offset2 {
    // The ranges have the following layout here:
    // [o1--------]
    //      [o2--------]
    let tail = offset2 - offset1;
    // Copy the tail from offset1 into offset2
    // [o1-][tail1]
    //      [o2-][tail2]
    // This needs to happen in the reverse order so that the later
    // values at offset1 are not mangled in the process of copying.
    // Same as memmove.
    for ii in (tail..len).rev() {
        input[offset2 + ii] = input[offset1 + ii];
    }

    // After this, the layout is the following:
    // [o1-][xxxxx]
    //      [o2-][tail1]
    len = tail;
    } else if offset2 < offset1 && offset2 + len >= offset1 {
    // The ranges have the following layout here:
    //      [o1--------]
    // [o2--------]
    let head = len - (offset1 - offset2);
    // Copy the head from offset1 into offset2
    //      [head1][o1-]
    // [head2][o2-]
    for ii in 0..head {
        input[offset2 + ii] = input[offset1 + ii];
    }

    // After this, the layout is the following:
    //      [xxxxx][o1-]
    // [head1][o2-]
    offset1 += head;
    offset2 += head;
    len     -= head;
    }

    // At this point, the ranges are non-overlapping
    // and the swap can be done in a naive way.
    for ii in 0..len {
    input.swap(offset1 + ii, offset2 + ii);
    }

    input.to_vec()
}

/// Copy bytes from one location in the input and overwrite them at another
/// location in the input
fn copy(input: &mut [u8], seed: u64) -> Vec<u8> {
    // Nothing to do on an empty input
    if input.is_empty() {
        return input.to_vec();
    }

    // Pick a source and destination for a copy
    let src    = rand_offset(input, seed);
    let srcrem = input.len() - src;
    let dst    = rand_offset(input, seed);
    let dstrem = input.len() - dst;

    // Pick a random length up to the max for both offsets
    let len = rand_number_in_range(1,core::cmp::min(srcrem, dstrem), seed);

    // Perform a copy inplace in the input
    overwrite_inplace(input, src, len, dst)
}

/// Take the bytes from `source` for `len` bytes in the input, and copy
/// them to `dest`
fn overwrite_inplace(input: &mut [u8], source: usize, len: usize, dest: usize) -> Vec<u8> {

    let mut new_input = input.to_owned();

    // Nothing to do
    if len == 0 || source == dest { return new_input.to_vec(); }

    if source < dest {
        // Copy forwards
        for ii in 0..len {
            new_input[dest + ii] = new_input[source + ii];
        }

        new_input.to_vec()
    } else {
        // Copy backwards
        for ii in (0..len).rev() {
            new_input[dest + ii] = new_input[source + ii];
        }

        new_input.to_vec()
    }
}

/// Take one location of the input and splice it into another
fn inter_splice(input: &mut [u8], seed: u64) -> Vec<u8> {
    // Nothing to do on an empty input
    if input.is_empty() {
        return input.to_vec();
    }

    // Pick a source and destination for an insertion
    let src    = rand_offset(input, seed);
    let srcrem = input.len() - src;
    let dst    = rand_offset(input, seed);

    // Pick a random length
    let len = rand_number_in_range(1,srcrem, seed);

    // Perform an insertion inplace in the input
    insert_inplace(input, src, len, dst)
}

/// Take the bytes from `source` for `len` bytes in the input, and insert
/// a copy of them at `dest`
fn insert_inplace(input: &mut [u8], source: usize, len: usize, dest: usize)-> Vec<u8> {
    // Nothing to do
    if len == 0 || source == dest { return input.to_vec(); }

    // Cap the insertion to the max input size
    let mut new_len = len;
    if MAX_INPUT_SIZE < input.len() {
        new_len = core::cmp::min(len, MAX_INPUT_SIZE);
    } else {
        new_len = core::cmp::min(len, MAX_INPUT_SIZE - input.len());
    }

    // Create an interator to splice into the input
    let rep = core::iter::repeat(b'\0').take(new_len);

    // Expand at `dest` with `rep`, making room for the copy
    let mut input_vec = input.to_vec();
    input_vec.splice(dest..=dest, rep);

    // Determine where the splice occurred
    let split_point = dest.saturating_sub(source).min(new_len);

    for ii in 0..split_point {
        input_vec[dest + ii] = input_vec[source + ii];
    }
    
    for ii in split_point..len {
        input_vec[dest + ii] = input_vec[source + ii + new_len];
    }

    let new_input = &input_vec;

    new_input.to_vec()
}

/// Create 1 or 2 random bytes and insert them into the input
fn insert_rand(input: &mut [u8], seed: u64) -> Vec<u8> {

    // Pick some random values
    let bytes = [rand_number_in_range(0,255, seed) as u8, rand_number_in_range(0,255, seed) as u8];


    // Pick a random offset and length
    let offset = rand_offset(input, seed);
    let len = rand_number_in_range(1,2, seed);

    // Insert the bytes
    insert(input, offset, &bytes[..len])
}

/// Insert `buf` at `offset` in the input. `buf` will be truncated to
/// ensure the input stays within the maximum input size
fn insert<'a>(input: &'a[u8], offset: usize, buf: &[u8]) -> Vec<u8> {
    let mut len = 0;
    // Make sure we don't expand past the maximum input size
    if MAX_INPUT_SIZE < input.len() {
        len = core::cmp::min(buf.len(), MAX_INPUT_SIZE);
    } else {
        len = core::cmp::min(buf.len(), MAX_INPUT_SIZE - input.len());
    };

    // Splice in the `buf`
    input.to_vec().splice(offset..offset, buf[..len].iter().copied());

    input.to_vec()
    
}

/// Create 1 or 2 random bytes and overwrite them at a location in the
/// input
fn overwrite_rand(input: &mut [u8], seed: u64) -> Vec<u8> {
    // Nothing to do on an empty input
    if input.is_empty() {
        return input.to_vec();
    }

    // Pick some random values
    let bytes = [rand_number_in_range(0,255, seed) as u8, rand_number_in_range(0,255, seed) as u8];

    // Pick a random offset and length
    let offset = rand_offset(input, seed);
    let len = core::cmp::min(input.len() - offset, 2);
    let len = rand_number_in_range(1,len, seed);

    // Overwrite the bytes
    let new_input = input;
    overwrite(new_input, offset, &bytes[..len])
}

/// Overwrite the bytes in the input with `buf` at `offset`. If `buf`
/// goes out of bounds of the input the `buf` will be truncated and the
/// copy will stop.
fn overwrite<'a>(input: &'a mut [u8], offset: usize, buf: &[u8]) -> Vec<u8> {
    // Get the slice that we may overwrite
    let target = &mut input[offset..];

    // Get the length to overwrite
    let len = core::cmp::min(buf.len(), target.len());

    // Overwrite the bytes
    target[..len].copy_from_slice(&buf[..len]);

    target.to_vec()
}