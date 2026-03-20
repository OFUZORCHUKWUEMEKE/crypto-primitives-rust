//! Sumcheck protocol for a single multilinear polynomial.
//!
//! ## What sumcheck does (in one sentence)
//!
//! The prover convinces the verifier that `Σ_{x ∈ {0,1}^n} f(x) = C`
//! without the verifier having to sum all `2^n` evaluations themselves.
//!
//! ## How it works
//!
//! Each round the prover fixes all variables except one, sums f over the
//! remaining boolean hypercube, and sends back a univariate polynomial.
//! The verifier checks consistency, picks a random challenge, and the
//! polynomial shrinks by one variable each round.
//!
//! After `n` rounds, the verifier holds a single evaluation claim which
//! they can check against the polynomial directly (the "oracle query").
//!

use crate::dense::DenseMLEPolynomial;
use crate::error::MLEError;
use ark_ff::Field;

// ─────────────────────────────────────────────────────────────────────────────
// Univariate polynomial
// ─────────────────────────────────────────────────────────────────────────────

/// A univariate polynomial stored as coefficients.
///
/// `coeffs[i]` is the coefficient of `tⁱ`.
///
/// So `[3, 5]` represents `3 + 5t`.
/// And `[1, 0, 2]` represents `1 + 2t²`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnivariatePoly<F: Field> {
    pub coeffs: Vec<F>,
}

impl<F: Field> UnivariatePoly<F> {
    /// Create from a list of coefficients.
    pub fn new(coeffs: Vec<F>) -> Self {
        Self { coeffs }
    }

    /// Evaluate the polynomial at point `t`.
    ///
    /// Uses Horner's method: `c₀ + t(c₁ + t(c₂ + …))`
    /// Fast — no repeated exponentiation.
    pub fn eval(&self, t: F) -> F {
        self.coeffs
            .iter()
            .rev()
            .fold(F::zero(), |acc, &c| acc * t + c)
    }

    /// Convenience: build a linear polynomial from its values at 0 and 1.
    ///
    /// `s(0) = a`, `s(1) = b`  →  coefficients: `[a, b-a]`
    /// because `a + (b-a)·t` gives `a` at t=0 and `b` at t=1.
    pub fn from_evals_at_01(s0: F, s1: F) -> Self {
        Self {
            coeffs: vec![s0, s1 - s0],
        }
    }

    /// The sum s(0) + s(1). Used in the sumcheck consistency check.
    pub fn sum_at_0_and_1(&self) -> F {
        self.eval(F::zero()) + self.eval(F::one())
    }

    /// Degree of the polynomial.
    pub fn degree(&self) -> usize {
        self.coeffs.len().saturating_sub(1)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Proof structure
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct SumcheckProof<F: Field> {
    /// The claimed sum: `Σ_{x ∈ {0,1}^n} f(x)`.
    pub claimed_sum: F,

    /// One univariate polynomial per round (one per variable).
    pub round_polys: Vec<UnivariatePoly<F>>,

    /// The verifier's random challenges — one per round.
    /// In a real protocol these come from the verifier (or Fiat-Shamir).
    pub challenges: Vec<F>,

    /// The final evaluation `f(r₀, r₁, …, rₙ₋₁)` at the challenge point.
    pub final_eval: F,
}

// ─────────────────────────────────────────────────────────────────────────────
// Prover
// ─────────────────────────────────────────────────────────────────────────────

/// Runs the sumcheck prover for `Σ_{x ∈ {0,1}^n} f(x)`.
///
/// Takes the polynomial directly as a `DenseMLEPolynomial` and uses its
/// built-in methods for summing and folding.
///
/// `challenges` are the verifier's random field elements — one per variable.
/// In a non-interactive setting, supply them from a Fiat-Shamir hash.
///
/// # What happens each round
///
/// The round polynomial for variable `xᵢ` is:
///
/// ```text
/// sᵢ(t) = Σ_{x_{i+1},…,xₙ ∈ {0,1}} f(r₀,…,r_{i-1}, t, x_{i+1},…,xₙ)
/// ```
///
/// Since `f` is multilinear, `sᵢ` is linear (degree 1) in `t`.
/// We evaluate at `t=0` and `t=1`, giving us the two coefficients.
pub fn prove<F: Field>(
    poly: &DenseMLEPolynomial<F>,
    challenges: &[F],
) -> Result<SumcheckProof<F>, MLEError> {
    let num_vars = poly.num_vars;
    assert_eq!(
        challenges.len(),
        num_vars,
        "need one challenge per variable"
    );

    let claimed_sum = poly.sum_over_hypercube();

    let mut current = poly.clone();
    let mut round_polys = Vec::with_capacity(num_vars);

    for &r in challenges {
        // s(0) = sum of the first half  (variable = 0)
        // s(1) = sum of the second half (variable = 1)
        let [s0, s1] = current.sumcheck_round();

        round_polys.push(UnivariatePoly::from_evals_at_01(s0, s1));

        // Bind the variable — shrinks polynomial by one variable.
        current = current.fix_variable(r)?;
    }

    // One entry remains: f(r₀, …, rₙ₋₁)
    let final_eval = current.evals[0];

    Ok(SumcheckProof {
        claimed_sum,
        round_polys,
        challenges: challenges.to_vec(),
        final_eval,
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// Verifier
// ─────────────────────────────────────────────────────────────────────────────

/// Verify a sumcheck proof against the original polynomial.
///
/// The verifier checks each round polynomial for consistency, then
/// confirms the final evaluation against the polynomial's own `evaluate()`.
pub fn verify<F: Field>(
    proof: &SumcheckProof<F>,
    poly: &DenseMLEPolynomial<F>,
) -> Result<bool, MLEError> {
    let num_vars = poly.num_vars;

    if proof.round_polys.len() != num_vars {
        return Ok(false);
    }
    if proof.challenges.len() != num_vars {
        return Ok(false);
    }

    let mut expected = proof.claimed_sum;

    for (round_poly, &r) in proof.round_polys.iter().zip(proof.challenges.iter()) {
        // s(0) + s(1) must equal what the previous round gave us.
        if round_poly.sum_at_0_and_1() != expected {
            return Ok(false);
        }
        // Next expected = s(r)
        expected = round_poly.eval(r);
    }

    // Final: last folded value must match the actual polynomial at the challenge point.
    let oracle_val = poly.evaluate(&proof.challenges)?;
    Ok(expected == proof.final_eval && proof.final_eval == oracle_val)
}
