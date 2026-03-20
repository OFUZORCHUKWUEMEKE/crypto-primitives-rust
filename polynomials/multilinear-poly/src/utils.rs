/// Returns 2^n, panicking cleanly if n > 63.
#[inline(always)]
pub fn two_pow(n: usize) -> usize {
    assert!(n <= 63, "num_vars {n} would overflow usize");
    1usize << n
}

/// Bit-reverse an index of `bits` width.
///
/// Used to convert between little-endian and big-endian variable ordering when
/// interfacing with external libraries that disagree on convention.
#[inline(always)]
pub fn bit_reverse(mut x: usize, bits: usize) -> usize {
    let mut result = 0;
    for _ in 0..bits {
        result = (result << 1) | (x & 1);
        x >>= 1;
    }
    result
}

/// Return the index in the evaluation table for a given binary assignment.
///
/// Encoding (little-endian): index = b_0 + 2*b_1 + 4*b_2 + ...
/// So variable 0 is the *least-significant* bit.
#[inline(always)]
pub fn index_from_bits(bits: &[bool]) -> usize {
    bits.iter()
        .rev()
        .fold(0usize, |acc, &b| (acc << 1) | b as usize)
    // revert: little-endian means b_0 is LSB
    // actually we want b_0 + 2*b_1 ... so:
}

/// True little-endian index: b_0 is bit 0 (LSB).
#[inline(always)]
pub fn le_index_from_bits(bits: &[bool]) -> usize {
    bits.iter()
        .enumerate()
        .fold(0usize, |acc, (i, &b)| acc | ((b as usize) << i))
}
