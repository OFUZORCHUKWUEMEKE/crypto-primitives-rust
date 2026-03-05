use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldElement {
    pub num: u128,
    pub prime: u128,
}

impl FieldElement {
    pub fn new(num: u128, prime: u128) -> Self {
        assert!(
            num < prime,
            "Num {} not in field range 0 to {}",
            num,
            prime - 1
        );
        Self { num, prime }
    }

    /// Modular exponentiation using the square-and-multiply algorithm
    pub fn pow(&self, exponent: u128) -> Self {
        let mut n = exponent;
        let mut base = self.num;
        let mut result = 1u128;

        while n > 0 {
            if n % 2 == 1 {
                result = (result * base) % self.prime;
            }
            base = (base * base) % self.prime;
            n /= 2;
        }

        Self {
            num: result,
            prime: self.prime,
        }
    }

    /// Modular inverse using the Extended Euclidean Algorithm
    pub fn inv(&self) -> Self {
        let mut t = 0i128;
        let mut newt = 1i128;
        let mut r = self.prime as i128;
        let mut newr = self.num as i128;

        while newr != 0 {
            let quotient = r / newr;

            let temp_t = t - quotient * newt;
            t = newt;
            newt = temp_t;

            let temp_r = r - quotient * newr;
            r = newr;
            newr = temp_r;
        }

        assert!(r <= 1, "self.num and self.prime are not co-prime");

        if t < 0 {
            t += self.prime as i128;
        }

        Self {
            num: t as u128,
            prime: self.prime,
        }
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        assert_eq!(
            self.prime, other.prime,
            "Cannot add two numbers in different Fields"
        );
        let num = (self.num + other.num) % self.prime;
        Self {
            num,
            prime: self.prime,
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        assert_eq!(
            self.prime, other.prime,
            "Cannot subtract two numbers in different Fields"
        );
        let num = if self.num >= other.num {
            self.num - other.num
        } else {
            self.prime - (other.num - self.num)
        };
        Self {
            num,
            prime: self.prime,
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        assert_eq!(
            self.prime, other.prime,
            "Cannot multiply two numbers in different Fields"
        );
        let num = (self.num * other.num) % self.prime;
        Self {
            num,
            prime: self.prime,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(7, 13);
        assert_eq!(a, b);
        let c = FieldElement::new(6, 13);
        assert_ne!(a, c);
    }

    #[test]
    fn test_add() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(12, 13);
        assert_eq!(a + b, FieldElement::new(6, 13));
    }

    #[test]
    fn test_sub() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(12, 13);
        assert_eq!(a - b, FieldElement::new(8, 13));
    }

    #[test]
    fn test_mul() {
        let a = FieldElement::new(3, 13);
        let b = FieldElement::new(12, 13);
        assert_eq!(a * b, FieldElement::new(10, 13));
    }

    #[test]
    fn test_pow() {
        let a = FieldElement::new(3, 13);
        assert_eq!(a.pow(3), FieldElement::new(1, 13));
    }

    #[test]
    fn test_inv() {
        let a = FieldElement::new(3, 13);
        assert_eq!(a.inv(), FieldElement::new(9, 13)); // 3 * 9 = 27 = 1 mod 13
    }

    #[test]
    fn test_inv_large() {
        let prime = 223;
        let a = FieldElement::new(15, prime);
        let inv_a = a.inv();
        assert_eq!(a * inv_a, FieldElement::new(1, prime));
    }
}
