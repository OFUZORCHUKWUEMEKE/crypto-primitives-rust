# Multilinear Polynomials

A **multilinear polynomial** is a polynomial in multiple variables where each variable has degree **at most 1**. These are the central primitive in sumcheck-based ZK protocols (Spartan, Lasso, HyperPlonk, etc.).

---

## 1. Math Background

### Definition

A multilinear polynomial in *n* variables over a field **F** is:

```
f(x₁, x₂, …, xₙ) = Σ   cₑ · x₁^e₁ · x₂^e₂ · … · xₙ^eₙ
                    e∈{0,1}ⁿ
```

where each exponent `eᵢ ∈ {0, 1}`. No variable ever appears squared.

**Example** with 2 variables:
```
f(x₁, x₂) = c₀₀ + c₁₀·x₁ + c₀₁·x₂ + c₁₁·x₁·x₂
             ↑       ↑         ↑         ↑
           (0,0)   (1,0)     (0,1)     (1,1)
```

This has **4 = 2²** coefficients — one for each vertex of the Boolean hypercube `{0,1}²`.

### The Boolean Hypercube

The **Boolean hypercube** `{0,1}ⁿ` is the set of all *n*-bit binary strings. It has `2ⁿ` elements.

```
n = 1:  {0, 1}                          → 2 vertices
n = 2:  {(0,0), (0,1), (1,0), (1,1)}    → 4 vertices
n = 3:  {(0,0,0), ..., (1,1,1)}         → 8 vertices
```

**Key insight**: A multilinear polynomial in *n* variables is **uniquely determined** by its values at the `2ⁿ` points of the Boolean hypercube. This is because there are exactly `2ⁿ` coefficients, and the evaluation at `2ⁿ` distinct points pins them all down.

### Evaluation Table Representation

Instead of storing coefficients, we store the polynomial as a **table of evaluations** over the Boolean hypercube:

```
evaluations[i] = f(b₁, b₂, …, bₙ)

where (b₁, b₂, …, bₙ) is the binary representation of index i
```

For `n = 2`:
```
Index 0  →  (0, 0)  →  evaluations[0] = f(0, 0)
Index 1  →  (1, 0)  →  evaluations[1] = f(1, 0)
Index 2  →  (0, 1)  →  evaluations[2] = f(0, 1)
Index 3  →  (1, 1)  →  evaluations[3] = f(1, 1)
```

> **Why bit-decompose in this order?** Index `i`'s bit `k` (counting from LSB = bit 0) tells you the value of `xₖ₊₁`. Bit 0 of `i` → `x₁`, bit 1 of `i` → `x₂`, etc.

### Multilinear Extension (MLE)

Given *any* function `f: {0,1}ⁿ → F`, there exists a **unique** multilinear polynomial that agrees with `f` on every point of the Boolean hypercube. This is called the **multilinear extension** of `f`.

The formula is:

```
f̃(x₁, …, xₙ) = Σ          f(b₁, …, bₙ) · Πᵢ [ xᵢ·bᵢ + (1 - xᵢ)·(1 - bᵢ) ]
              (b₁,…,bₙ)∈{0,1}ⁿ
```

The product `Πᵢ [xᵢ·bᵢ + (1-xᵢ)·(1-bᵢ)]` is the **multilinear Lagrange basis** — it equals 1 when `(x₁,…,xₙ) = (b₁,…,bₙ)` and 0 at every other hypercube vertex.

### Evaluation at an Arbitrary Point

To evaluate `f̃(r₁, r₂, …, rₙ)` for arbitrary field elements `rᵢ` (not just bits), we interpolate *one variable at a time*:

```
For each variable xₖ being set to rₖ:
  For each pair of entries that differ only in bit k:
    new[...0...] = (1 - rₖ) · old[...0...] + rₖ · old[...1...]
```

This halves the table at each step: `2ⁿ → 2ⁿ⁻¹ → … → 1`.

**Walk-through** for `n = 2`, evaluating at `(r₁, r₂)`:

```
Start: evals = [f(0,0), f(1,0), f(0,1), f(1,1)]      (4 values)

Fix x₁ = r₁:
  evals'[0] = (1-r₁)·evals[0] + r₁·evals[1]    →  f(r₁, 0)
  evals'[1] = (1-r₁)·evals[2] + r₁·evals[3]    →  f(r₁, 1)
                                                    (2 values)

Fix x₂ = r₂:
  result    = (1-r₂)·evals'[0] + r₂·evals'[1]   →  f(r₁, r₂)
                                                    (1 value ✓)
```

### Partial Evaluation

**Partial evaluation** means fixing *one* variable to a specific value while leaving the rest free. If we fix `x₁ = r`, we get a new multilinear polynomial in `n-1` variables.

This is the same "halving" step from full evaluation — but we stop after one round.

---

## 2. The Struct

```rust
use ark_ff::Field;

/// A multilinear polynomial in `num_vars` variables over field F.
///
/// Stored as evaluations over the Boolean hypercube {0,1}^num_vars.
///   evaluations[i] = f(b₁, b₂, …, bₙ)
///   where (b₁, …, bₙ) is the binary decomposition of i
///     (bit 0 = x₁, bit 1 = x₂, …)
///
/// The length of `evaluations` must always be 2^num_vars.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultilinearPolynomial<F: Field> {
    /// Number of variables.
    pub num_vars: usize,
    /// Evaluations at each vertex of {0,1}^num_vars.
    /// Length = 2^num_vars.
    pub evaluations: Vec<F>,
}
```

> **Why store evaluations instead of coefficients?**
> In ZK protocols (sumcheck, GKR), we almost always work with evaluation representations. They make partial evaluation a simple linear-time operation, and converting from a truth table to an MLE is "free" — the truth table *is* the evaluation table.

---

## 3. Core Operations

### Constructor & helpers

```rust
impl<F: Field> MultilinearPolynomial<F> {
    /// Create a multilinear polynomial from its evaluations on {0,1}^num_vars.
    ///
    /// Panics if evaluations.len() != 2^num_vars.
    pub fn new(num_vars: usize, evaluations: Vec<F>) -> Self {
        assert_eq!(
            evaluations.len(),
            1 << num_vars,  // 2^num_vars
            "evaluations length must be 2^num_vars"
        );
        Self { num_vars, evaluations }
    }

    /// The zero polynomial in `num_vars` variables.
    pub fn zero(num_vars: usize) -> Self {
        Self {
            num_vars,
            evaluations: vec![F::zero(); 1 << num_vars],
        }
    }

    /// Check if every evaluation is zero.
    pub fn is_zero(&self) -> bool {
        self.evaluations.iter().all(|e| e.is_zero())
    }

    /// Number of variables.
    pub fn num_vars(&self) -> usize {
        self.num_vars
    }

    /// Number of evaluations (= 2^num_vars).
    pub fn len(&self) -> usize {
        self.evaluations.len()
    }
}
```

---

### Evaluation at an arbitrary point

```rust
impl<F: Field> MultilinearPolynomial<F> {
    /// Evaluate f(r₁, r₂, …, rₙ) at an arbitrary point in F^n.
    ///
    /// Works by fixing one variable at a time, halving the table:
    ///   Round 1: fix x₁ = r₁  →  table shrinks from 2ⁿ to 2ⁿ⁻¹
    ///   Round 2: fix x₂ = r₂  →  table shrinks from 2ⁿ⁻¹ to 2ⁿ⁻²
    ///   …
    ///   Round n: fix xₙ = rₙ  →  table shrinks to 1 entry = result
    ///
    /// Panics if point.len() != num_vars.
    pub fn evaluate(&self, point: &[F]) -> F {
        assert_eq!(point.len(), self.num_vars, "point dimension mismatch");

        // Start with a copy of the evaluation table
        let mut evals = self.evaluations.clone();

        // Fix one variable at a time
        for r in point.iter() {
            let half = evals.len() / 2;
            let mut new_evals = Vec::with_capacity(half);

            for j in 0..half {
                // evals[j]        = f(…, xₖ=0, …)
                // evals[j + half] = f(…, xₖ=1, …)
                //
                // Interpolate: (1 - r) · f(…,0,…) + r · f(…,1,…)
                let val = (F::one() - r) * evals[j] + *r * evals[j + half];
                new_evals.push(val);
            }

            evals = new_evals;
        }

        // After fixing all variables, only one value remains
        evals[0]
    }
}
```

**Walk-through** for `f(x₁, x₂)` with evaluations `[1, 3, 5, 7]`:

```
This means:
  f(0,0) = 1,  f(1,0) = 3,  f(0,1) = 5,  f(1,1) = 7

Evaluate at (r₁, r₂) = (2, 3):

Round 1 — fix x₁ = 2:
  evals = [1, 3, 5, 7],  half = 2
  j=0: (1-2)·1 + 2·3 = -1 + 6 = 5
  j=1: (1-2)·5 + 2·7 = -5 + 14 = 9
  evals = [5, 9]

Round 2 — fix x₂ = 3:
  half = 1
  j=0: (1-3)·5 + 3·9 = -10 + 27 = 17
  evals = [17]

Result: f(2, 3) = 17
```

> Note: these are field elements, so in a real finite field the arithmetic wraps modulo *p*. The example uses integers for clarity.

---

### Partial Evaluation

```rust
impl<F: Field> MultilinearPolynomial<F> {
    /// Fix variable x₁ to value `r`, returning a new polynomial
    /// in (num_vars - 1) variables.
    ///
    /// This is one round of the full evaluation algorithm.
    pub fn partial_evaluate(&self, r: &F) -> Self {
        assert!(self.num_vars > 0, "can't partial-evaluate 0-variable poly");

        let half = self.evaluations.len() / 2;
        let mut new_evals = Vec::with_capacity(half);

        for j in 0..half {
            let val = (F::one() - r) * self.evaluations[j]
                    + *r * self.evaluations[j + half];
            new_evals.push(val);
        }

        Self {
            num_vars: self.num_vars - 1,
            evaluations: new_evals,
        }
    }
}
```

**Connection to the sumcheck protocol**: each round of sumcheck fixes one variable using partial evaluation. After *n* rounds, you have a constant (0-variable polynomial) that the verifier can check.

---

## 4. Arithmetic

### Addition

```rust
use std::ops::{Add, Sub, Neg};

impl<F: Field> Add for MultilinearPolynomial<F> {
    type Output = Self;

    /// Add two multilinear polynomials point-wise.
    /// They must have the same number of variables.
    fn add(self, other: Self) -> Self {
        assert_eq!(self.num_vars, other.num_vars, "variable count mismatch");

        let evaluations = self.evaluations.iter()
            .zip(other.evaluations.iter())
            .map(|(a, b)| *a + *b)
            .collect();

        Self {
            num_vars: self.num_vars,
            evaluations,
        }
    }
}
```

### Negation

```rust
impl<F: Field> Neg for MultilinearPolynomial<F> {
    type Output = Self;

    fn neg(self) -> Self {
        let evaluations = self.evaluations.into_iter()
            .map(|e| -e)
            .collect();

        Self {
            num_vars: self.num_vars,
            evaluations,
        }
    }
}
```

### Subtraction

```rust
impl<F: Field> Sub for MultilinearPolynomial<F> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}
```

### Scalar Multiplication

```rust
impl<F: Field> MultilinearPolynomial<F> {
    /// Multiply every evaluation by a scalar.
    pub fn scale(&self, scalar: &F) -> Self {
        let evaluations = self.evaluations.iter()
            .map(|e| *e * scalar)
            .collect();

        Self {
            num_vars: self.num_vars,
            evaluations,
        }
    }
}
```

> **Why no `Mul` between two multilinear polynomials?**
> Multiplying two multilinear polynomials gives a polynomial where variables can have degree 2 — it's no longer multilinear. In ZK protocols, if you need to multiply, you use a different representation (e.g., sumcheck over the product).

---

## 5. Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::MontFp;
    use ark_bn254::Fr;

    #[test]
    fn test_new_and_basics() {
        // f(x₁, x₂): 4 evaluations for 2 variables
        let f = MultilinearPolynomial::<Fr>::new(2, vec![
            MontFp!("1"), MontFp!("3"), MontFp!("5"), MontFp!("7"),
        ]);
        assert_eq!(f.num_vars(), 2);
        assert_eq!(f.len(), 4);
        assert!(!f.is_zero());
    }

    #[test]
    fn test_zero_polynomial() {
        let z = MultilinearPolynomial::<Fr>::zero(3);
        assert!(z.is_zero());
        assert_eq!(z.len(), 8); // 2^3
    }

    #[test]
    fn test_evaluate_at_hypercube_vertex() {
        // f(x₁, x₂) with evals [1, 3, 5, 7]
        // f(0,0)=1, f(1,0)=3, f(0,1)=5, f(1,1)=7
        let f = MultilinearPolynomial::<Fr>::new(2, vec![
            MontFp!("1"), MontFp!("3"), MontFp!("5"), MontFp!("7"),
        ]);

        // Evaluate at (0, 0) → should be 1
        let val = f.evaluate(&[Fr::zero(), Fr::zero()]);
        assert_eq!(val, MontFp!("1"));

        // Evaluate at (1, 0) → should be 3
        let val = f.evaluate(&[Fr::one(), Fr::zero()]);
        assert_eq!(val, MontFp!("3"));

        // Evaluate at (1, 1) → should be 7
        let val = f.evaluate(&[Fr::one(), Fr::one()]);
        assert_eq!(val, MontFp!("7"));
    }

    #[test]
    fn test_evaluate_at_arbitrary_point() {
        // f(x₁, x₂) with evals [1, 3, 5, 7]
        // Using the MLE formula:
        //   f(r₁, r₂) = (1-r₁)(1-r₂)·1 + r₁(1-r₂)·3 + (1-r₁)r₂·5 + r₁r₂·7
        //
        // At (2, 3):
        //   = (1-2)(1-3)·1 + 2(1-3)·3 + (1-2)·3·5 + 2·3·7
        //   = (-1)(-2)·1  + 2(-2)·3  + (-1)(3)·5 + 6·7
        //   = 2 - 12 - 15 + 42
        //   = 17
        let f = MultilinearPolynomial::<Fr>::new(2, vec![
            MontFp!("1"), MontFp!("3"), MontFp!("5"), MontFp!("7"),
        ]);

        let result = f.evaluate(&[MontFp!("2"), MontFp!("3")]);
        assert_eq!(result, MontFp!("17"));
    }

    #[test]
    fn test_partial_evaluate() {
        // f(x₁, x₂) with evals [1, 3, 5, 7]
        // Fix x₁ = 2:
        //   g(x₂) = f(2, x₂)
        //   g(0) = (1-2)·1 + 2·3 = 5
        //   g(1) = (1-2)·5 + 2·7 = 9
        let f = MultilinearPolynomial::<Fr>::new(2, vec![
            MontFp!("1"), MontFp!("3"), MontFp!("5"), MontFp!("7"),
        ]);

        let g = f.partial_evaluate(&MontFp!("2"));

        assert_eq!(g.num_vars(), 1);
        assert_eq!(g.evaluations, vec![MontFp!("5"), MontFp!("9")]);

        // Now evaluate g at x₂ = 3:  (1-3)·5 + 3·9 = 17
        let result = g.evaluate(&[MontFp!("3")]);
        assert_eq!(result, MontFp!("17"));
    }

    #[test]
    fn test_full_eval_matches_sequential_partial_eval() {
        // Evaluating f(r₁, r₂) directly should equal
        // partial_evaluate(r₁) then evaluate(r₂)
        let f = MultilinearPolynomial::<Fr>::new(2, vec![
            MontFp!("10"), MontFp!("20"), MontFp!("30"), MontFp!("40"),
        ]);

        let r1 = MontFp!("5");
        let r2 = MontFp!("7");

        let direct = f.evaluate(&[r1, r2]);
        let via_partial = f.partial_evaluate(&r1).evaluate(&[r2]);

        assert_eq!(direct, via_partial);
    }

    #[test]
    fn test_addition() {
        let f = MultilinearPolynomial::<Fr>::new(1, vec![
            MontFp!("2"), MontFp!("5"),
        ]);
        let g = MultilinearPolynomial::<Fr>::new(1, vec![
            MontFp!("10"), MontFp!("20"),
        ]);

        let sum = f + g;
        assert_eq!(sum.evaluations, vec![MontFp!("12"), MontFp!("25")]);
    }

    #[test]
    fn test_subtraction_yields_zero() {
        let f = MultilinearPolynomial::<Fr>::new(2, vec![
            MontFp!("1"), MontFp!("2"), MontFp!("3"), MontFp!("4"),
        ]);
        let diff = f.clone() - f;
        assert!(diff.is_zero());
    }

    #[test]
    fn test_scalar_multiplication() {
        let f = MultilinearPolynomial::<Fr>::new(1, vec![
            MontFp!("3"), MontFp!("7"),
        ]);
        let scaled = f.scale(&MontFp!("10"));
        assert_eq!(scaled.evaluations, vec![MontFp!("30"), MontFp!("70")]);
    }

    #[test]
    fn test_single_variable() {
        // f(x₁) with evals [a, b]  →  f(r) = (1-r)·a + r·b
        // evals = [4, 10]
        // f(0) = 4, f(1) = 10
        // f(3) = (1-3)·4 + 3·10 = -8 + 30 = 22
        let f = MultilinearPolynomial::<Fr>::new(1, vec![
            MontFp!("4"), MontFp!("10"),
        ]);
        assert_eq!(f.evaluate(&[MontFp!("3")]), MontFp!("22"));
    }
}
```

---

## Summary

| Component | What it does | Complexity |
|---|---|---|
| `MultilinearPolynomial<F>` | Stores `evaluations: Vec<F>` of length `2ⁿ` | O(2ⁿ) space |
| `evaluate` | Fixes all *n* vars, halving each round | O(2ⁿ) time |
| `partial_evaluate` | Fixes 1 variable | O(2ⁿ⁻¹) time |
| `add` / `sub` | Point-wise on evaluation tables | O(2ⁿ) time |
| `scale` | Multiply all evaluations by scalar | O(2ⁿ) time |

### Comparison with Univariate

| | Univariate | Multilinear |
|---|---|---|
| **Variables** | 1 | *n* |
| **Storage** | Coefficient vector | Evaluation table |
| **Size** | *d + 1* entries | *2ⁿ* entries |
| **Eval cost** | O(d) | O(2ⁿ) |
| **Used in** | KZG, FFT, PLONK | Sumcheck, GKR, Spartan |
