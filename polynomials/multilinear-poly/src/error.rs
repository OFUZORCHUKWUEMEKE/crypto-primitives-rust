use thiserror::Error;

#[derive(Debug, Error)]
pub enum MLEError {
    #[error("evals length {0} is not a power of two")]
    NotPowerOfTwo(usize),

    #[error("point length {point} != num_vars {vars}")]
    PointLenMismatch { point: usize, vars: usize },

    #[error("polynomial num_vars mismatch: {lhs} vs {rhs}")]
    NumVarsMismatch { lhs: usize, rhs: usize },

    #[error("cannot fix variable on a constant polynomial (0 variables)")]
    NoVariables,
}