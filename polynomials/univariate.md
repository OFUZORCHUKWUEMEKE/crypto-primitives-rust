# Univariate Polynomials

A **univariate polynomial** is a polynomial in a single variable. This is the bread-and-butter primitive in ZK proof systems — used everywhere from commitment schemes (KZG) to FFT-based protocols.

---

## 1. Math Background

A univariate polynomial of degree *d* over a field **F** is:

```
p(x) = a₀ + a₁·x + a₂·x² + … + aₐ·xᵈ
```

where each coefficient `aᵢ ∈ F`.

### Key properties

| Property | Description |
|---|---|
| **Degree** | The highest power of *x* with a non-zero coefficient |
| **Evaluation** | Plug in a value `x = r` → get `p(r) ∈ F` |
| **Zero polynomial** | All coefficients are zero; degree is undefined (or −∞ by convention) |
| **Addition** | Add corresponding coefficients |
| **Multiplication** | Convolve coefficient vectors; `deg(p·q) = deg(p) + deg(q)` |
| **Schwartz-Zippel** | A non-zero degree-*d* poly has at most *d* roots in **F** |

### Horner's Method (efficient evaluation)

Instead of computing each power of `x` separately, **Horner's method** rewrites:

```
p(x) = a₀ + x·(a₁ + x·(a₂ + … + x·aₐ))
```

This uses only **d multiplications** and **d additions** — no exponentiation needed.

**Step-by-step** for `p(x) = 3 + 2x + 5x²`:
```
Start from the highest coefficient:
  result = 5
  result = result * x + 2  →  5x + 2
  result = result * x + 3  →  5x² + 2x + 3   ✓
```

### Lagrange Interpolation

Given *d+1* distinct points `{(x₀, y₀), (x₁, y₁), …, (xₐ, yₐ)}`, there is **exactly one** polynomial of degree ≤ *d* passing through all of them:

```
p(x) = Σᵢ yᵢ · Lᵢ(x)

where Lᵢ(x) = Πⱼ≠ᵢ (x - xⱼ) / (xᵢ - xⱼ)
```

Each `Lᵢ` is a **Lagrange basis polynomial** — it equals 1 at `xᵢ` and 0 at every other `xⱼ`.

---

## 2. The Struct

We store a polynomial as a vector of coefficients, where index `i` holds the coefficient of `xⁱ`.

```rust
use ark_ff::Field;

/// A univariate polynomial over a field F.
///
/// Internally stored as a coefficient vector:
///   coeffs[i] = coefficient of x^i
///
/// Example: 3 + 2x + 5x²  →  coeffs = [3, 2, 5]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnivariatePolynomial<F: Field> {
    /// coeffs[i] is the coefficient of x^i.
    /// The vector may have trailing zeros after arithmetic,
    /// but `truncate()` normalizes it.
    pub coeffs: Vec<F>,
}
```

> **Why `Vec<F>` and not a fixed-size array?**
> Polynomial degree isn't known at compile time (especially after multiplication, which increases degree). A `Vec` lets us grow naturally.

---

## 3. Core Operations

### Constructor & helpers

```rust
impl<F: Field> UnivariatePolynomial<F> {
    /// Create a new polynomial from coefficients.
    /// coeffs[0] is the constant term, coeffs[d] is the leading term.
    pub fn new(coeffs: Vec<F>) -> Self {
        let mut poly = Self { coeffs };
        poly.truncate(); // remove trailing zeros
        poly
    }

    /// The zero polynomial (additive identity).
    pub fn zero() -> Self {
        Self { coeffs: vec![] }
    }

    /// Check if this is the zero polynomial.
    pub fn is_zero(&self) -> bool {
        self.coeffs.is_empty()
    }

    /// Degree of the polynomial.
    /// Returns `None` for the zero polynomial.
    pub fn degree(&self) -> Option<usize> {
        if self.is_zero() {
            None
        } else {
            Some(self.coeffs.len() - 1)
        }
    }

    /// Remove trailing zero coefficients so that
    /// the last element is always non-zero (or the vec is empty).
    fn truncate(&mut self) {
        while self.coeffs.last() == Some(&F::zero()) {
            self.coeffs.pop();
        }
    }
}
```

**What `truncate` does**: after adding `[1, 2, 3]` and `[-1, -2, -3]` you'd get `[0, 0, 0]`. Truncation strips those trailing zeros so the result is the empty vec (= zero polynomial). This keeps `degree()` correct.

---

### Evaluation (Horner's method)

```rust
impl<F: Field> UnivariatePolynomial<F> {
    /// Evaluate p(x) at a given point using Horner's method.
    ///
    /// Horner's rewrites:
    ///   a₀ + a₁x + a₂x² + a₃x³
    /// as:
    ///   a₀ + x(a₁ + x(a₂ + x·a₃))
    ///
    /// This requires only `d` multiplications and `d` additions.
    pub fn evaluate(&self, x: &F) -> F {
        if self.is_zero() {
            return F::zero();
        }

        // Walk coefficients from highest to lowest
        let mut result = F::zero();
        for coeff in self.coeffs.iter().rev() {
            result = result * x + coeff;
            //       ^^^^^^^^^^   ^^^^^
            //       shift by x   add next coefficient
        }
        result
    }
}
```

**Walk-through** for `p(x) = 3 + 2x + 5x²`, evaluating at `x = 4`:
```
coeffs = [3, 2, 5]   (iterate reversed → 5, 2, 3)

step 0:  result = 0
step 1:  result = 0 * 4 + 5  = 5
step 2:  result = 5 * 4 + 2  = 22
step 3:  result = 22 * 4 + 3 = 91

Check: 3 + 2(4) + 5(16) = 3 + 8 + 80 = 91  ✓
```

---

## 4. Arithmetic Trait Implementations

### Addition

```rust
use std::ops::{Add, Sub, Neg, Mul};

impl<F: Field> Add for UnivariatePolynomial<F> {
    type Output = Self;

    /// Add two polynomials by adding corresponding coefficients.
    /// If they have different lengths, the shorter one is
    /// implicitly padded with zeros.
    fn add(self, other: Self) -> Self {
        let len = std::cmp::max(self.coeffs.len(), other.coeffs.len());
        let mut result = Vec::with_capacity(len);

        for i in 0..len {
            let a = self.coeffs.get(i).copied().unwrap_or(F::zero());
            let b = other.coeffs.get(i).copied().unwrap_or(F::zero());
            result.push(a + b);
        }

        Self::new(result) // constructor calls truncate()
    }
}
```

### Negation

```rust
impl<F: Field> Neg for UnivariatePolynomial<F> {
    type Output = Self;

    /// Negate every coefficient.
    fn neg(self) -> Self {
        let coeffs = self.coeffs.into_iter().map(|c| -c).collect();
        Self::new(coeffs)
    }
}
```

### Subtraction

```rust
impl<F: Field> Sub for UnivariatePolynomial<F> {
    type Output = Self;

    /// p - q  =  p + (-q)
    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}
```

### Multiplication

```rust
impl<F: Field> Mul for UnivariatePolynomial<F> {
    type Output = Self;

    /// Multiply two polynomials (convolution of coefficient vectors).
    ///
    /// If deg(p) = m and deg(q) = n, then deg(p*q) = m + n.
    /// The result has (m + n + 1) coefficients.
    ///
    /// Each result[k] = Σ  p[i] * q[k-i]   for valid i
    fn mul(self, other: Self) -> Self {
        if self.is_zero() || other.is_zero() {
            return Self::zero();
        }

        let result_len = self.coeffs.len() + other.coeffs.len() - 1;
        let mut result = vec![F::zero(); result_len];

        for (i, a) in self.coeffs.iter().enumerate() {
            for (j, b) in other.coeffs.iter().enumerate() {
                result[i + j] += *a * *b;
            }
        }

        Self::new(result)
    }
}
```

**Why `result[i + j]`?**  When you multiply `aᵢ·xⁱ` by `bⱼ·xʲ`, you get `(aᵢ·bⱼ)·x^(i+j)`. So the product contributes to index `i+j` in the result.

---

## 5. Lagrange Interpolation

```rust
impl<F: Field> UnivariatePolynomial<F> {
    /// Given a set of points (xᵢ, yᵢ), return the unique polynomial
    /// of degree ≤ n-1 that passes through all of them.
    ///
    /// Uses the Lagrange interpolation formula:
    ///   p(x) = Σᵢ  yᵢ · Lᵢ(x)
    ///
    /// where Lᵢ(x) = Πⱼ≠ᵢ  (x - xⱼ) / (xᵢ - xⱼ)
    ///
    /// Panics if any two x-values are the same.
    pub fn interpolate(points: &[(F, F)]) -> Self {
        assert!(!points.is_empty(), "need at least one point");

        let n = points.len();
        let mut result = Self::zero();

        for i in 0..n {
            let (xi, yi) = points[i];

            // Build Lagrange basis polynomial Lᵢ(x)
            // Start with the constant polynomial "1"
            let mut basis = Self::new(vec![F::one()]);

            for j in 0..n {
                if i == j {
                    continue;
                }

                let (xj, _) = points[j];

                // denominator = xᵢ - xⱼ  (a field element, must be non-zero)
                let denom = xi - xj;
                assert!(denom != F::zero(), "duplicate x-values");
                let denom_inv = denom.inverse().unwrap();

                // numerator polynomial = (x - xⱼ)  →  coeffs = [-xⱼ, 1]
                let factor = Self::new(vec![-xj, F::one()]);

                // basis *= (x - xⱼ) / (xᵢ - xⱼ)
                // We multiply by the polynomial, then scale by 1/denom
                basis = basis * factor;
                basis.coeffs.iter_mut().for_each(|c| *c *= denom_inv);
            }

            // Scale basis Lᵢ(x) by yᵢ
            basis.coeffs.iter_mut().for_each(|c| *c *= yi);

            // Accumulate into result
            result = result + basis;
        }

        result
    }
}
```

**Walk-through** for points `(1, 2)` and `(3, 10)`:
```
We want the line through these two points.

i=0: xi=1, yi=2
  basis starts as [1]
  j=1: factor = (x - 3) = [-3, 1],  denom = 1-3 = -2
       basis = [1] * [-3, 1] = [-3, 1]
       scale by 1/(-2): [3/2, -1/2]
  scale by y₀=2: [3, -1]

i=1: xi=3, yi=10
  basis starts as [1]
  j=0: factor = (x - 1) = [-1, 1],  denom = 3-1 = 2
       basis = [1] * [-1, 1] = [-1, 1]
       scale by 1/2: [-1/2, 1/2]
  scale by y₁=10: [-5, 5]

result = [3, -1] + [-5, 5] = [-2, 4]

So p(x) = -2 + 4x
Check: p(1) = -2+4 = 2 ✓,  p(3) = -2+12 = 10 ✓
```

---

## 6. Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::MontFp;
    // Using the BN254 scalar field as our concrete field
    use ark_bn254::Fr;

    #[test]
    fn test_new_and_degree() {
        // p(x) = 3 + 2x + 5x²   →   degree 2
        let p = UnivariatePolynomial::<Fr>::new(vec![
            MontFp!("3"), MontFp!("2"), MontFp!("5"),
        ]);
        assert_eq!(p.degree(), Some(2));
    }

    #[test]
    fn test_zero_polynomial() {
        let z = UnivariatePolynomial::<Fr>::zero();
        assert!(z.is_zero());
        assert_eq!(z.degree(), None);
    }

    #[test]
    fn test_trailing_zeros_truncated() {
        // [1, 0, 0] should become [1]  →  degree 0
        let p = UnivariatePolynomial::<Fr>::new(vec![
            MontFp!("1"), Fr::zero(), Fr::zero(),
        ]);
        assert_eq!(p.degree(), Some(0));
        assert_eq!(p.coeffs.len(), 1);
    }

    #[test]
    fn test_evaluate_horner() {
        // p(x) = 3 + 2x + 5x²
        let p = UnivariatePolynomial::<Fr>::new(vec![
            MontFp!("3"), MontFp!("2"), MontFp!("5"),
        ]);
        // p(4) = 3 + 8 + 80 = 91
        let result = p.evaluate(&MontFp!("4"));
        assert_eq!(result, MontFp!("91"));
    }
    

    #[test]
    fn test_evaluate_zero_poly() {
        let z = UnivariatePolynomial::<Fr>::zero();
        assert_eq!(z.evaluate(&MontFp!("42")), Fr::zero());
    }

    #[test]
    fn test_addition() {
        // p(x) = 1 + 2x       q(x) = 3 + 4x + 5x²
        let p = UnivariatePolynomial::<Fr>::new(vec![
            MontFp!("1"), MontFp!("2"),
        ]);
        let q = UnivariatePolynomial::<Fr>::new(vec![
            MontFp!("3"), MontFp!("4"), MontFp!("5"),
        ]);
        // p + q = 4 + 6x + 5x²
        let sum = p + q;
        assert_eq!(sum.coeffs, vec![
            MontFp!("4"), MontFp!("6"), MontFp!("5"),
        ]);
    }

    #[test]
    fn test_subtraction() {
        let p = UnivariatePolynomial::<Fr>::new(vec![
            MontFp!("5"), MontFp!("3"),
        ]);
        let q = UnivariatePolynomial::<Fr>::new(vec![
            MontFp!("5"), MontFp!("3"),
        ]);
        // p - p = 0
        let diff = p - q;
        assert!(diff.is_zero());
    }

    #[test]
    fn test_multiplication() {
        // (1 + x) * (1 + x) = 1 + 2x + x²
        let p = UnivariatePolynomial::<Fr>::new(vec![
            MontFp!("1"), MontFp!("1"),
        ]);
        let q = p.clone();
        let product = p * q;
        assert_eq!(product.coeffs, vec![
            MontFp!("1"), MontFp!("2"), MontFp!("1"),
        ]);
    }

    #[test]
    fn test_negation() {
        let p = UnivariatePolynomial::<Fr>::new(vec![
            MontFp!("3"), MontFp!("7"),
        ]);
        let neg_p = -p.clone();
        // p + (-p) = 0
        let sum = p + neg_p;
        assert!(sum.is_zero());
    }

    #[test]
    fn test_lagrange_interpolation() {
        // Interpolate through (1, 2) and (3, 10)
        // Expected: p(x) = -2 + 4x
        let points = vec![
            (MontFp!("1"), MontFp!("2")),
            (MontFp!("3"), MontFp!("10")),
        ];
        let p = UnivariatePolynomial::<Fr>::interpolate(&points);

        // Verify it passes through both points
        assert_eq!(p.evaluate(&MontFp!("1")), MontFp!("2"));
        assert_eq!(p.evaluate(&MontFp!("3")), MontFp!("10"));

        // Verify degree
        assert_eq!(p.degree(), Some(1));
    }

    #[test]
    fn test_lagrange_quadratic() {
        // Three points define a unique degree-2 polynomial
        // (0, 1), (1, 0), (2, 1)  →  p(x) = 1 - 2x + x²
        let points = vec![
            (Fr::zero(), MontFp!("1")),
            (MontFp!("1"), Fr::zero()),
            (MontFp!("2"), MontFp!("1")),
        ];
        let p = UnivariatePolynomial::<Fr>::interpolate(&points);

        assert_eq!(p.evaluate(&Fr::zero()), MontFp!("1"));
        assert_eq!(p.evaluate(&MontFp!("1")), Fr::zero());
        assert_eq!(p.evaluate(&MontFp!("2")), MontFp!("1"));
        assert_eq!(p.degree(), Some(2));
    }
}
```

---

## Summary

| Component | What it does | Complexity |
|---|---|---|
| `UnivariatePolynomial<F>` | Stores `coeffs: Vec<F>` | O(d) space |
| `evaluate` (Horner) | Evaluates at a point | O(d) time |
| `add` / `sub` | Coefficient-wise ops | O(d) time |
| `mul` | Convolution | O(d²) time (naïve) |
| `interpolate` | Lagrange from points | O(n²) time |

> **Next up**: [multilinear.md](./multilinear.md) — multilinear polynomials over the Boolean hypercube.
