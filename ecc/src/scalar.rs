use crate::point::Point;
use std::ops::Mul;

/// Scalar multiplication: k * P
/// Uses the Double-and-Add algorithm which works in O(log k) time.
impl Mul<Point> for u128 {
    type Output = Point;

    fn mul(self, point: Point) -> Point {
        let mut k = self;
        let mut current_point = point;
        let mut result = Point::infinity(point.a, point.b);

        while k > 0 {
            // "Add" step: if the least significant bit is 1, add the current point to the result
            if k % 2 == 1 {
                result = result + current_point;
            }
            // "Double" step: double the current point for the next bit
            current_point = current_point + current_point;

            // Shift right to process the next bit
            k /= 2;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::FieldElement;

    #[test]
    fn test_scalar_mul() {
        // secp256k1 toy parameters
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);

        let x1 = FieldElement::new(47, prime);
        let y1 = FieldElement::new(71, prime);
        let p1 = Point::new(Some(x1), Some(y1), a, b);

        // Test 2 * P1 == P1 + P1
        assert_eq!(2 * p1, p1 + p1);

        // Test 3 * P1 == P1 + P1 + P1
        assert_eq!(3 * p1, p1 + p1 + p1);

        // Test that 10 * P1 works via scalar_mul
        let x10 = FieldElement::new(154, prime);
        let y10 = FieldElement::new(150, prime);
        let p10 = Point::new(Some(x10), Some(y10), a, b);

        assert_eq!(10 * p1, p10);
    }

    #[test]
    fn test_scalar_mul_group_order() {
        // For the curve y^2 = x^3 + 7 over F_223,
        // the point (47, 71) generates a sub-group of order 21.
        // Therefore, 21 * P should equal the Point at Infinity.
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);

        let x1 = FieldElement::new(47, prime);
        let y1 = FieldElement::new(71, prime);
        let g = Point::new(Some(x1), Some(y1), a, b);
        let inf = Point::infinity(a, b);

        assert_eq!(21 * g, inf);
    }
}
