use thiserror::Error;

#[derive(Debug, Error)]
pub enum PolyError {
    #[error("evaluations length {got} does not match 2^{num_vars} = {expected}")]
    InvalidLength {
        num_vars: usize,
        expected: usize,
        got: usize,
    },

    #[error("dimension mismatch: left has {left} vars, right has {right} vars")]
    DimensionMismatch {
        left: usize,
        right: usize,
    },

    #[error("point has {got} elements but polynomial has {expected} variables")]
    PointDimensionMismatch {
        expected: usize,
        got: usize,
    },

    #[error("cannot partial-evaluate a polynomial with 0 variables")]
    EmptyPolynomial,
}
