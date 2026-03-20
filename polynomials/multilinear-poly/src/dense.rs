use core::fmt;

use crate::{
    error::MLEError,
    utils::{self, two_pow},
};
use ark_ff::Field;

#[derive(Clone, PartialEq, Eq)]
pub struct DenseMLEPolynomial<F: Field> {
    /// Number of variables `n`.
    pub num_vars: usize,
    /// 2^n evaluations over {0,1}^n in little-endian index order.
    pub evals: Vec<F>,
}

impl<F: Field> DenseMLEPolynomial<F> {
    #[inline]
    pub fn new(evals: Vec<F>) -> Self {
        let len = evals.len();
        assert!(
            len.is_power_of_two(),
            "evals length {len} must be a power of 2"
        );
        let num_vars = len.trailing_zeros() as usize;
        Self { num_vars, evals }
    }

    pub fn try_new(evals: Vec<F>) -> Result<Self, MLEError> {
        let len = evals.len();
        if !len.is_power_of_two() {
            return Err(MLEError::NotPowerOfTwo(len));
        }
        Ok(Self::new(evals))
    }

    pub fn constant(c: F, num_vars: usize) -> Self {
        Self {
            num_vars,
            evals: vec![c; two_pow(num_vars)],
        }
    }

    pub fn one(num_vars: usize) -> Self {
        Self::constant(F::ONE, num_vars)
    }

    /// Number of evaluations (= 2^num_vars).
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.evals.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.evals.is_empty()
    }

    pub fn evaluate(&self, point: &[F]) -> Result<F, MLEError> {
        if point.len() != self.num_vars {
            return Err(MLEError::PointLenMismatch {
                point: point.len(),
                vars: self.num_vars,
            });
        }
        Ok(self.evaluate_inner(point))
    }

    #[inline]
    fn evaluate_inner(&self, point: &[F]) -> F {
        let mut table = self.evals.clone();
        fold_in_place(&mut table, point);
        table[0]
    }

    /// Evaluate at a point on the boolean hypercube {0,1}ⁿ.
    ///
    /// Equivalent to `evals[le_index]`, but provided for clarity.
    #[inline]
    pub fn evaluate_at_vertex(&self, bits: &[bool]) -> Result<F, MLEError> {
        if bits.len() != self.num_vars {
            return Err(MLEError::PointLenMismatch {
                point: bits.len(),
                vars: self.num_vars,
            });
        }
        let idx = bits
            .iter()
            .enumerate()
            .fold(0usize, |acc, (i, &b)| acc | ((b as usize) << i));
        Ok(self.evals[idx])
    }

    pub fn fix_variable(&self, value: F) -> Result<Self, MLEError> {
        if self.num_vars == 0 {
            return Err(MLEError::NoVariables);
        }
        let half = self.evals.len() >> 1;
        let one_minus_r = F::ONE - value;

        let new_evals: Vec<F> = (0..half)
            .map(|i| {
                // evals[i]        = f(0, b₁, b₂, …)   [x₀ = 0]
                // evals[i + half] = f(1, b₁, b₂, …)   [x₀ = 1]
                one_minus_r * self.evals[i] + value * self.evals[i + half]
            })
            .collect();

        Ok(Self {
            num_vars: self.num_vars - 1,
            evals: new_evals,
        })
    }

    pub fn fix_variables(&self, values: &[F]) -> Result<Self, MLEError> {
        let k = values.len();
        if k > self.num_vars {
            return Err(MLEError::PointLenMismatch {
                point: k,
                vars: self.num_vars,
            });
        }
        let mut table = self.evals.clone();
        fold_in_place(&mut table[..], values);
        let new_len = table.len() >> k;
        table.truncate(new_len);

        Ok(Self {
            num_vars: self.num_vars - k,
            evals: table,
        })
    }

    pub fn sum_over_hypercube(&self) -> F {
        self.evals.iter().copied().sum()
    }

    pub fn sumcheck_round(&self) -> [F; 2] {
        let half = self.evals.len() >> 1;
        let (s0, s1) = (0..half).fold((F::ZERO, F::ZERO), |(a0, a1), i| {
            (a0 + self.evals[i], a1 + self.evals[i + half])
        });
        [s0, s1]
    }
}

fn fold_in_place<F: Field>(table: &mut [F], challenges: &[F]) {
    let n = challenges.len();
    let mut size = table.len();

    for &r in challenges.iter().take(n) {
        size >>= 1;

        #[cfg(feature = "parallel")]
        {
            use rayon::prelude::*;
            // Only pay rayon overhead for large tables.
            if size >= 1 << 10 {
                let (lo, hi) = table.split_at_mut(size);
                lo.par_iter_mut().zip(hi.par_iter()).for_each(|(l, h)| {
                    *l += r * (*h - *l);
                });
                continue;
            }
        }

        // Sequential fallback (also the only path without `parallel` feature).
        let (lo, hi) = table.split_at_mut(size);
        for (l, h) in lo.iter_mut().zip(hi.iter()) {
            *l += r * (*h - *l);
        }
    }
}

impl<F: Field + fmt::Display> fmt::Debug for DenseMLEPolynomial<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DenseMLEPolynomial {{ num_vars: {}, evals: {:?} }}",
            self.num_vars, self.evals
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr;
    use ark_ff::One;

    fn fr(x: u64) -> Fr {
        Fr::from(x)
    }

    fn sample_2var() -> DenseMLEPolynomial<Fr> {
        DenseMLEPolynomial::new(vec![fr(1), fr(2), fr(3), fr(4)])
    }

    #[test]
    fn construction() {
        let p = sample_2var();
        assert_eq!(p.num_vars, 2);
        assert_eq!(p.len(), 4);
    }

    #[test]
    fn three_vars() {
        // f defined over {0,1}³ with evals = [0,1,2,3,4,5,6,7]
        let evals: Vec<Fr> = (0u64..8).map(fr).collect();
        let p = DenseMLEPolynomial::new(evals);
        assert_eq!(p.num_vars, 3);

        // Evaluate at (1,1,1) — should give evals[0b111] = 7
        let result = p.evaluate(&[Fr::ONE, Fr::ONE, Fr::ONE]).unwrap();
        assert_eq!(result, fr(7));

        // // Evaluate at (0,0,0) — should give evals[0] = 0
        // let result = p.evaluate(&[Fr::new(0), Fr::new(0), Fr::new(0)]).unwrap();
        // assert_eq!(result, fr(0));
    }
}
