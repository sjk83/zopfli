use std::{slice};

use libc::{c_uint, size_t, c_int};

use katajainen::length_limited_code_lengths;

pub fn zopfli_lengths_to_symbols(lengths_ptr: *const c_uint, n: size_t, maxbits: c_uint, symbols: *mut c_uint) {
    let lengths = unsafe { slice::from_raw_parts(lengths_ptr, n) };
    let lengths_size_t = lengths.iter().map(|&len| len as size_t).collect::<Vec<_>>();

    let syms = lengths_to_symbols(&lengths_size_t, maxbits);
    for i in 0..n {
        unsafe {
            *symbols.offset(i as isize) = syms[i];
        }
    }
}

/// Converts a series of Huffman tree bitlengths, to the bit values of the symbols.
pub fn lengths_to_symbols(lengths: &[usize], maxbits: c_uint) -> Vec<c_uint> {
    let mut bl_count = vec![0; (maxbits + 1) as usize];
    let mut next_code = vec![0; (maxbits + 1) as usize];
    let n = lengths.len();

    let mut symbols = vec![0; n];

    // 1) Count the number of codes for each code length. Let bl_count[N] be the
    // number of codes of length N, N >= 1. */
    for i in 0..n {
        assert!(lengths[i] <= maxbits as usize);
        bl_count[lengths[i]] += 1;
    }
    // 2) Find the numerical value of the smallest code for each code length.
    let mut code = 0;
    bl_count[0] = 0;
    for bits in 1..(maxbits + 1) {
        code = (code + bl_count[(bits - 1) as usize]) << 1;
        next_code[bits as usize] = code;
    }
    // 3) Assign numerical values to all codes, using consecutive values for all
    // codes of the same length with the base values determined at step 2.
    for i in 0..n {
        let len = lengths[i];
        if len != 0 {
            symbols[i] = next_code[len];
            next_code[len] += 1;
        }
    }
    symbols
}

/// Calculates the bitlengths for the Huffman tree, based on the counts of each
/// symbol.
pub fn zopfli_calculate_bit_lengths(frequencies: *const size_t, n: usize, maxbits: c_int, bitlengths: *mut c_uint) {
    let freqs = unsafe { slice::from_raw_parts(frequencies, n) };
    let result = length_limited_code_lengths(freqs, maxbits);

    for (i, res) in result.into_iter().enumerate() {
        unsafe {
            *bitlengths.offset(i as isize) = res as c_uint;
        }
    }
}
