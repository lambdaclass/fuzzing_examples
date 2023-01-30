#[macro_use]
extern crate honggfuzz;
use cita_trie::MemoryDB;
use cita_trie::{PatriciaTrie, Trie};
use hasher::HasherKeccak;
use patricia_merkle_tree::PatriciaMerkleTree;
use sha3::Keccak256;
use proptest::prelude::*;
use std::sync::Arc;
use crate::prop::collection::vec;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {


            println!("input used:");
            println!("{:?}", data);

            let mut tree = PatriciaMerkleTree::<&[u8], &[u8], Keccak256>::new();
            
            let root_hash = tree.compute_hash().as_slice().to_vec();
        
            let memdb = Arc::new(MemoryDB::new(true));
            let hasher = Arc::new(HasherKeccak::new());
            let mut trie = PatriciaTrie::new(Arc::clone(&memdb), Arc::clone(&hasher));

            let inputs = mutate(data);


            for input in inputs {
                tree.insert(input, input);
                trie.insert(input.to_vec(), input.to_vec()).unwrap();
            };
            
            let reference_root = trie.root().unwrap();
            println!("{:?}", reference_root);
            println!("{:?}", root_hash);
        
            assert_eq!(
            reference_root,
            root_hash
            )
        });
    }
}

struct Rng {
    /// The RNG's seed and state
    seed: u64,

    /// If set, `rand_exp` behaves the same as `rand`
    exp_disabled: bool,
}

impl Rng {
    /// Generate a random number
    #[inline]
    fn next(&mut self) -> u64 {
        let val = self.seed;
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 17;
        self.seed ^= self.seed << 43;
        val
    }

    /// Generates a random number with uniform distribution in the range of
    /// [min, max]
    #[inline]
    fn rand(&mut self, min: usize, max: usize) -> usize {
        // Make sure the range is sane
        assert!(max >= min, "Bad range specified for rand()");

        // If there is no range, just return `min`
        if min == max {
            return min;
        }
        
        // If the range is unbounded, just return a random number
        if min == 0 && max == core::usize::MAX {
            return self.next() as usize;
        }

        // Pick a random number in the range
        min + (self.next() as usize % (max - min + 1))
    }
    
    /// Generates a random number with exponential distribution in the range of
    /// [min, max] with a worst case deviation from uniform of 0.5x. Meaning
    /// this will always return uniform at least half the time.
    #[inline]
    fn rand_exp(&mut self, min: usize, max: usize) -> usize {
        // If exponential random is disabled, fall back to uniform
        if self.exp_disabled {
            return self.rand(min, max);
        }

        if self.rand(0, 1) == 0 {
            // Half the time, provide uniform
            self.rand(min, max)
        } else {
            // Pick an exponentially difficult random number
            let x = self.rand(min, max);
            self.rand(min, x)
        }
    }
}


fn mutate(input: &[u8]) -> Vec<&[u8]> {
    /// List of mutation strategies which do not require an input database
    const STRATEGIES: &[fn()] = &[
        shrink,
        expand,
        add_sub,
        set,
        swap,
        copy,
        inter_splice,
        // Mutator::insert_rand,
        // Mutator::overwrite_rand,
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

    let mut vec = vec![input];

    let rng = Rng {
        seed:         0x12640367f4b7ea35,
        exp_disabled: false,
    };

    for _ in 0..5 {
        // Pick a random mutation strategy
        let sel = rng.rand(0, STRATEGIES.len() - 1);

        // Save the old state of the exponential random and randomly disable
        // the exponential random
        let old_exp_state = rng.exp_disabled;

        if rng.rand(0, 1) == 0 {
            rng.exp_disabled = true;
        }
            
        // Get the strategy
        let strat = STRATEGIES[sel];

        // Run the mutation strategy
        vec.push(strat(input, rng));

        // Restore exponential random state to the old state
        rng.exp_disabled = old_exp_state;
        
    };

    vec
}

fn rand_exp(rnd: Rng, min: usize, max: usize) -> usize {

    if rnd.rand(0, 1) == 0 {
        // Half the time, provide uniform
        rnd.rand(min, max)
    } else {
        // Pick an exponentially difficult random number
        let x = rnd.rand(min, max);
        rnd.rand(min, x)
    }
}

fn rand_offset(rnd: Rng,input: &[u8]) -> usize {
    if !input.is_empty() {
        // just return a random index
        rand_exp(rnd, 0, input.len() - 1 as usize)
    } else {
        // Input is entirely empty, just return index 0 such that
        // things that insert into the input know that they should
        // just insert at 0.
        0
    }
}

/// Randomly delete a chunk of the input
fn shrink(input: &[u8], rng: Rng) {
    // Nothing to do on an empty input
    if input.is_empty() {
        return;
    }

    // Pick a random offset to remove data at
    let offset = rand_offset(rng, input);

    // Compute the number of bytes we could remove from this offset
    let can_remove = input.len() - offset;

    // Compute a maximum number of bytes to remove
    let max_remove = if rng.rand(0, 15) != 0 {
        // 15 in 16 chance of removing at most 16 bytes, this limits the
        // amount we remove in the most common case
        core::cmp::min(16, can_remove)
    } else {
        // 1 in 16 chance of removing a random amount of bytes to the end
        // of the input
        can_remove
    };

    // Pick the amount of bytes to remove
    let to_remove = rng.rand_exp(1, max_remove);

    // Remove the bytes from the input
    let _ = input.drain(offset..offset + to_remove);
}

fn expand(input: &[u8], rng: Rng) {
    
    // Pick a random offset to expand at
    let offset = rand_offset(rng, input);

    // Compute the number of bytes we could expand from this offset
    let can_expand = input.len() + offset;

    // Compute a maximum number of expansion bytes
    let max_expand = if rng.rand(0, 15) != 0 {
        // 15 in 16 chance of capping expansion to 16 bytes
        core::cmp::min(16, can_expand)
    } else {
        // 1 in 16 chance of uncapped expansion
        can_expand
    };

    // Create what to expand with
    let iter = core::iter::repeat(b'\0').take(rng.rand_exp(1, max_expand)); 
    
    // Expand at `offset` with `iter`
    input.splice(offset..offset, iter)
}

/// Add or subtract a random amount with a random endianness from a random
/// size `u8` through `u64`
fn add_sub(input: &[u8], rng: Rng) {
    // Nothing to do on an empty input
    if input.is_empty() {
        return;
    }

    // Pick an offset to corrupt at
    let offset = rand_offset(rng, input);

    // Get the remaining number of bytes in the input
    let remain = input.len() - offset;

    // Pick a random size of the add or subtract as a 1, 2, 4, or 8 byte
    // signed integer
    let intsize = match remain {
        1..=1                => 1,
        2..=3                => 1 << rng.rand(0, 1),
        4..=7                => 1 << rng.rand(0, 2),
        8..=core::usize::MAX => 1 << rng.rand(0, 3),
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
    let delta = rng.rand(0, range * 2) as i32 - range as i32;

    /// Macro to mutate bytes in the input as a `$ty`
    macro_rules! mutate {
        ($ty:ty) => {{
            // Interpret the `offset` as a `$ty`
            let tmp = <$ty>::from_ne_bytes(
                input[offset..offset + intsize].try_into().unwrap());

            // Apply the delta, interpreting the bytes as a random
            // endianness
            let tmp = if rng.rand(0, 1) == 0 {
                tmp.wrapping_add(delta as $ty)
            } else {
                tmp.swap_bytes().wrapping_add(delta as $ty).swap_bytes()
            };

            // Write the new value out to the input
            input[offset..offset + intsize].copy_from_slice(
                &tmp.to_ne_bytes())
        }}
    }

    // Apply the delta to the offset
    match intsize {
        1 => mutate!(u8),
        2 => mutate!(u16),
        4 => mutate!(u32),
        8 => mutate!(u64),
        _ => unreachable!(),
    };

}

/// Randomly replace a sequence of bytes with the same random character
/// repeated a random amount of times
fn set(input: &[u8], rng: Rng) {
    // Nothing to do on an empty input
    if input.is_empty() {
        return;
    }

    // Pick offset to memset at
    let offset = rand_offset(rng, input);

    // Pick random length to remainder of input
    let len = rng.rand_exp(1, input.len() - offset);

    // Pick the value to memset
    let chr = rng.rand(0, 255) as u8;

    // Replace the selected bytes at the offset with `chr`
    input[offset..offset + len].iter_mut().for_each(|x| *x = chr);
}

/// Swap two difference sequence of bytes in the input to different places
fn swap(input: &[u8], rng: Rng) {
    // Nothing to do on an empty input
    if input.is_empty() {
        return;
    }

    // Pick two random ranges in the input and calculate the remaining
    // bytes for them
    let src    = rand_offset(rng, input);
    let srcrem = input.len() - src;
    let dst    = rand_offset(rng, input);
    let dstrem = input.len() - dst;

    // Pick a random length up to the max for both offsets
    let len = rng.rand_exp(1, core::cmp::min(srcrem, dstrem));

    // Swap the ranges of bytes
    swap_ranges(input, src, dst, len);
}

/// Swap two ranges in an input buffer
fn swap_ranges(input: &[u8], mut offset1: usize, mut offset2: usize,
    mut len: usize) {
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
        vec[offset2 + ii] = vec[offset1 + ii];
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
        vec[offset2 + ii] = vec[offset1 + ii];
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
}

/// Copy bytes from one location in the input and overwrite them at another
/// location in the input
fn copy(input: &[u8], rng: Rng) {
    // Nothing to do on an empty input
    if input.is_empty() {
        return;
    }

    // Pick a source and destination for a copy
    let src    = rand_offset(rng, input);
    let srcrem = input.len() - src;
    let dst    = rand_offset(rng, input);
    let dstrem = input.len() - dst;

    // Pick a random length up to the max for both offsets
    let len = rng.rand_exp(1, core::cmp::min(srcrem, dstrem));

    // Perform a copy inplace in the input
    overwrite_inplace(input, src, len, dst);
}

/// Take the bytes from `source` for `len` bytes in the input, and copy
/// them to `dest`
fn overwrite_inplace(input: &[u8], source: usize, len: usize, dest: usize) {
    // Nothing to do
    if len == 0 || source == dest { return; }

    if source < dest {
        // Copy forwards
        for ii in 0..len {
            input[dest + ii] = input[source + ii];
        }
    } else {
        // Copy backwards
        for ii in (0..len).rev() {
            input[dest + ii] = input[source + ii];
        }
    }
}

/// Take one location of the input and splice it into another
fn inter_splice(input: &[u8], rng: Rng) {
    // Nothing to do on an empty input
    if input.is_empty() {
        return;
    }

    // Pick a source and destination for an insertion
    let src    = rand_offset(rng, input);
    let srcrem = input.len() - src;
    let dst    = rand_offset(rng, input);

    // Pick a random length
    let len = rng.rand_exp(1, srcrem);

    // Perform an insertion inplace in the input
    insert_inplace(input, src, len, dst);
}

/// Take the bytes from `source` for `len` bytes in the input, and insert
/// a copy of them at `dest`
fn insert_inplace(input: &[u8], source: usize, len: usize, dest: usize) {
    // Nothing to do
    if len == 0 || source == dest { return; }

    // Cap the insertion to the max input size
    let len = core::cmp::min(len, 128 - input.len());

    // Create an interator to splice into the input
    let rep = core::iter::repeat(b'\0').take(len);

    // Expand at `dest` with `rep`, making room for the copy
    input.splice(dest..dest, rep);

    // Determine where the splice occurred
    let split_point = dest.saturating_sub(source).min(len);

    for ii in 0..split_point {
        input[dest + ii] = input[source + ii];
    }
    
    for ii in split_point..len {
        input[dest + ii] = input[source + ii + len];
    }
}


