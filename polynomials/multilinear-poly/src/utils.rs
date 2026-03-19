use core::num;

use crate::error::PolyError;
use ark_ff::Field;

#[inline]
pub fn bit_of(index: usize, k: usize) -> usize {
    (index >> k) & 1
}

pub fn to_binary_field_vec<F: Field>(index: usize, num_bits: usize) -> Vec<F> {
    (0..num_bits)
        .map(|k| {
            if bit_of(index, k) == 1 {
                F::one()
            } else {
                F::zero()
            }
        })
        .collect()
}
