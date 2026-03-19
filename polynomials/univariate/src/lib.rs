use ark_ff::{Field, Zero};
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnivariatePolynomial<F: Field> {
    pub coeffs: Vec<F>,
}

impl<F: Field> UnivariatePolynomial<F> {
    pub fn new(coeffs: Vec<F>) -> Self {
        let mut poly = Self { coeffs };
        poly.truncate();
        poly
    }

    /// The zero polynomial (additive identity).
    pub fn zero() -> Self {
        Self { coeffs: vec![] }
    }

    pub fn is_zero(&self) -> bool {
        self.coeffs.is_empty()
    }

    pub fn degree(&self) -> Option<usize> {
        if self.is_zero() {
            None
        } else {
            Some(self.coeffs.len() - 1)
        }
    }

    pub fn truncate(&mut self) {
        while self.coeffs.last() == Some(&F::zero()) {
            self.coeffs.pop();
        }
    }
}

impl<F: Field> UnivariatePolynomial<F> {
    /// Evaluate p(x) at a given point using Horner's method.
    ///
    /// Horner's rewrites:
    ///   a₀ + a₁x + a₂x² + a₃x³
    /// as:
    ///   a₀ + x(a₁ + x(a₂ + x·a₃))

    pub fn evaluate(&self, x: &F) -> F {
        if self.is_zero() {
            return F::zero();
        }

        let mut result = F::zero();
        for coeff in self.coeffs.iter().rev() {
            result = result * x + coeff;
        }
        result
    }
}

impl<F: Field> Add for UnivariatePolynomial<F> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let len = std::cmp::max(self.coeffs.len(), other.coeffs.len());
        let mut result = Vec::with_capacity(len);

        for i in 0..len {
            let a = self.coeffs.get(i).copied().unwrap_or(F::zero());
            let b = other.coeffs.get(i).copied().unwrap_or(F::zero());
            result.push(a + b);
        }

        Self::new(result)
    }
}

impl<F: Field> Neg for UnivariatePolynomial<F> {
    type Output = Self;

    /// Negate every coefficient.
    fn neg(self) -> Self {
        let coeffs = self.coeffs.into_iter().map(|c| -c).collect();
        Self::new(coeffs)
    }
}

impl<F: Field> Sub for UnivariatePolynomial<F> {
    type Output = Self;

    /// p - q  =  p + (-q)
    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}

impl<F: Field> Mul for UnivariatePolynomial<F> {
    type Output = Self;

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

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr;
    use ark_ff::{Field, One, Zero};

    fn f(n: i64) -> Fr {
        if n >= 0 {
            Fr::from(n as u64)
        } else {
            -Fr::from((-n) as u64)
        }
    }

    fn poly(coeffs: &[i64]) -> UnivariatePolynomial<Fr> {
        UnivariatePolynomial::new(coeffs.iter().map(|&c| f(c)).collect())
    }

    #[test]
    fn test_new_strips_trailing_zeros() {
        // [1, 2, 0, 0] should truncate to [1, 2]
        let p = poly(&[1, 2, 0, 0]);
        assert_eq!(p.coeffs.len(), 2);
        assert_eq!(p.coeffs, vec![f(1), f(2)]);
    }

    #[test]
    fn test_new_all_zeros_becomes_zero_poly() {
        let p = poly(&[0, 0, 0]);
        assert!(p.is_zero());
    }
}
