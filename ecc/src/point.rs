use crate::field::FieldElement;
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    /// x-coordinate. If None, this represents the Point at Infinity (Identity element)
    pub x: Option<FieldElement>,
    /// y-coordinate. If None, this represents the Point at Infinity (Identity element)
    pub y: Option<FieldElement>,
    /// Curve coefficient 'a'
    pub a: FieldElement,
    /// Curve coefficient 'b'
    pub b: FieldElement,
}

impl Point {
    /// Create a new point on the elliptic curve y^2 = x^3 + ax + b
    pub fn new(x: Option<FieldElement>, y: Option<FieldElement>, a: FieldElement, b: FieldElement) -> Self {
        // Point at infinity
        if x.is_none() && y.is_none() {
            return Self { x, y, a, b };
        }

        let x_val = x.expect("x cannot be None if y is Some");
        let y_val = y.expect("y cannot be None if x is Some");

        assert_eq!(x_val.prime, y_val.prime, "x and y must be in the same field");
        assert_eq!(x_val.prime, a.prime, "Coordinates must be in the same field as curve parameters");
        assert_eq!(a.prime, b.prime, "Curve parameters must be in the same field");

        // Verify the point is actually on the curve: y^2 = x^3 + ax + b
        let y_squared = y_val.pow(2);
        let x_cubed = x_val.pow(3);
        let ax = a * x_val;
        let right_side = x_cubed + ax + b;

        assert_eq!(
            y_squared, right_side,
            "Point {:?}, {:?} is not on the curve",
            x_val, y_val
        );

        Self { x, y, a, b }
    }

    /// Helper to create a point at infinity for a given curve
    pub fn infinity(a: FieldElement, b: FieldElement) -> Self {
        Self { x: None, y: None, a, b }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        assert_eq!(self.a, other.a, "Points are not on the same curve");
        assert_eq!(self.b, other.b, "Points are not on the same curve");

        // Case 1: self is the Point at Infinity
        if self.x.is_none() {
            return other;
        }

        // Case 2: other is the Point at Infinity
        if other.x.is_none() {
            return self;
        }

        let x1 = self.x.unwrap();
        let y1 = self.y.unwrap();
        let x2 = other.x.unwrap();
        let y2 = other.y.unwrap();

        // Case 3: Points are additive inverses (same x, different y)
        // In a finite field, additive inverse means y1 == -y2 (or y1 + y2 == 0)
        // Note: FieldElement equality checks handle the underlying values
        if x1 == x2 && y1 != y2 {
            return Self::infinity(self.a, self.b);
        }

        // Case 4: Points are distinct (x1 != x2)
        if x1 != x2 {
            // Slope (s) = (y2 - y1) / (x2 - x1)
            let s = (y2 - y1) * (x2 - x1).inv();
            
            // x3 = s^2 - x1 - x2
            let x3 = s.pow(2) - x1 - x2;
            
            // y3 = s(x1 - x3) - y1
            let y3 = s * (x1 - x3) - y1;

            return Self {
                x: Some(x3),
                y: Some(y3),
                a: self.a,
                b: self.b,
            };
        }

        // Case 5: Points are the same (x1 == x2 and y1 == y2) - Point Doubling
        if self == other {
            // Case 5a: y coordinate is 0 (tangent line is vertical)
            // The slope goes to infinity.
            if y1.num == 0 {
                return Self::infinity(self.a, self.b);
            }

            // Case 5b: Standard point doubling
            // Slope (s) = (3x1^2 + a) / (2y1)
            let three = FieldElement::new(3, x1.prime);
            let two = FieldElement::new(2, x1.prime);
            
            let s = (three * x1.pow(2) + self.a) * (two * y1).inv();
            
            // x3 = s^2 - 2x1
            let x3 = s.pow(2) - (two * x1);
            
            // y3 = s(x1 - x3) - y1
            let y3 = s * (x1 - x3) - y1;

            return Self {
                x: Some(x3),
                y: Some(y3),
                a: self.a,
                b: self.b,
            };
        }

        unreachable!("All point addition cases should have been handled");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_on_curve() {
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime); // y^2 = x^3 + 7 over F_223 (secp256k1 toy version)

        let x1 = FieldElement::new(192, prime);
        let y1 = FieldElement::new(105, prime);
        let _p1 = Point::new(Some(x1), Some(y1), a, b); // Should not panic
    }

    #[test]
    #[should_panic(expected = "is not on the curve")]
    fn test_point_not_on_curve() {
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);

        // x=200, y=119 is on the curve. Let's mutate y so it panics.
        let x1 = FieldElement::new(200, prime);
        let y1_wrong = FieldElement::new(120, prime);
        let _p1 = Point::new(Some(x1), Some(y1_wrong), a, b);
    }

    #[test]
    fn test_point_addition_infinity() {
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);

        let x1 = FieldElement::new(192, prime);
        let y1 = FieldElement::new(105, prime);
        let p1 = Point::new(Some(x1), Some(y1), a, b);
        let inf = Point::infinity(a, b);

        assert_eq!(p1 + inf, p1);
        assert_eq!(inf + p1, p1);
    }

    #[test]
    fn test_point_addition_inverse() {
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);

        let x1 = FieldElement::new(192, prime);
        let y1 = FieldElement::new(105, prime);
        let y1_inv = FieldElement::new(prime - 105, prime);

        let p1 = Point::new(Some(x1), Some(y1), a, b);
        let p1_inv = Point::new(Some(x1), Some(y1_inv), a, b);
        let inf = Point::infinity(a, b);

        assert_eq!(p1 + p1_inv, inf);
    }

    #[test]
    fn test_point_addition_distinct() {
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);

        let x1 = FieldElement::new(192, prime);
        let y1 = FieldElement::new(105, prime);
        let p1 = Point::new(Some(x1), Some(y1), a, b);

        let x2 = FieldElement::new(17, prime);
        let y2 = FieldElement::new(56, prime);
        let p2 = Point::new(Some(x2), Some(y2), a, b);

        let x3 = FieldElement::new(170, prime);
        let y3 = FieldElement::new(142, prime);
        let p3 = Point::new(Some(x3), Some(y3), a, b);

        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn test_point_doubling() {
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);

        let x1 = FieldElement::new(47, prime);
        let y1 = FieldElement::new(71, prime);
        let p1 = Point::new(Some(x1), Some(y1), a, b);

        let x3 = FieldElement::new(36, prime);
        let y3 = FieldElement::new(111, prime);
        let p3 = Point::new(Some(x3), Some(y3), a, b);

        assert_eq!(p1 + p1, p3);
    }
}
