use ark_bls12_381::Fr;

use crate::commit;

pub fn explore_addition() {
    println!("=== Additive Homomorphism ===\n");

    let v1 = Fr::from(30u64);
    let r1 = Fr::from(111u64);

    let v2 = Fr::from(70u64);
    let r2 = Fr::from(222u64);

    // Commit to each value separately
    let c1 = commit::commit(&v1, &r1);
    let c2 = commit::commit(&v2, &r2);

    // Add the commitments (point addition)
    let c_sum = c1 + c2;
    // Commit to the sum of values with the sum of blinding factors
    let v_sum = v1 + v2; // 30 + 70 = 100
    let r_sum = r1 + r2; // 111 + 222 = 333
    let c_direct = commit::commit(&v_sum, &r_sum);
    // They must be equal!
    assert_eq!(c_sum, c_direct);

    println!("C1 = Commit(30, 111)");
    println!("C2 = Commit(70, 222)");
    println!("\nC1 + C2          = {:?}", c_sum);
    println!("Commit(100, 333) = {:?}", c_direct);
    println!("\n✓ C1 + C2 == Commit(v1+v2, r1+r2)");
    println!("  Addition works on encrypted values!");
}

/// Demonstrates subtraction:
///   C1 - C2 == Commit(v1 - v2, r1 - r2)
pub fn explore_subtraction() {
    println!("\n=== Subtraction ===\n");
    let v1 = Fr::from(100u64);
    let r1 = Fr::from(500u64);
    let v2 = Fr::from(40u64);
    let r2 = Fr::from(200u64);

    let c1 = commit::commit(&v1, &r1);
    let c2 = commit::commit(&v2, &r2);
    // Subtract commitments
    let c_diff = c1 - c2;

    // Commit to the difference
    let v_diff = v1 - v2; // 100 - 40 = 60
    let r_diff = r1 - r2; // 500 - 200 = 300
    let c_direct = commit::commit(&v_diff, &r_diff);

    println!("C1 = Commit(100, 500)");
    println!("C2 = Commit(40, 200)");
    println!("\nC1 - C2         = {:?}", c_diff);
    println!("Commit(60, 300) = {:?}", c_direct);
    println!("\n✓ C1 - C2 == Commit(v1-v2, r1-r2)");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_additive_homomorphism() {
        let v1 = Fr::from(42u64);
        let r1 = Fr::from(100u64);
        let v2 = Fr::from(58u64);
        let r2 = Fr::from(200u64);
        let c1 = commit::commit(&v1, &r1);
        let c2 = commit::commit(&v2, &r2);
        let c_sum = c1 + c2;
        let c_direct = commit::commit(&(v1 + v2), &(r1 + r2));
        assert_eq!(c_sum, c_direct);
    }

    #[test]
    fn test_subtraction_homomorphism() {
        let v1 = Fr::from(100u64);
        let r1 = Fr::from(500u64);
        let v2 = Fr::from(40u64);
        let r2 = Fr::from(200u64);
        let c1 = commit::commit(&v1, &r1);
        let c2 = commit::commit(&v2, &r2);
        let c_diff = c1 - c2;
        let c_direct = commit::commit(&(v1 - v2), &(r1 - r2));
        assert_eq!(c_diff, c_direct);
    }
}
