use ark_bls12_381::{Fr, G1Projective};
use ark_ff::PrimeField;
use ark_std::UniformRand;

use crate::generator;

/// Commits to a value `v` with blinding factor `r`.
///
/// C = v·G + r·H
///
/// - `v` is your secret value (what you're committing to)
/// - `r` is a random blinding factor (hides the value)
/// - Returns a curve point — the commitment
pub fn commit(v: &Fr, r: &Fr) -> G1Projective {
    let g = generator::generator_g();
    let h = generator::generator_h();

    // C = v*G + r*H
    g * v + h * r
}

pub fn verify(v: &Fr, r: &Fr, c: &G1Projective) -> bool {
    let expected = commit(v, r);
    expected == *c
}

pub fn explore_commit_verify() {
    println!("=== Basic Commit & Verify ===\n");
    let v = Fr::from(42u64); // our secret value
    let r = Fr::from(12345u64); // our blinding factor

    // Commit
    let c = commit(&v, &r);
    println!("Value:      v = 42");
    println!("Blinding:   r = 12345");
    println!("Commitment: C = {:?}", c);

    // Verify with correct opening
    let is_valid = verify(&v, &r, &c);
    assert!(is_valid);
    println!("\n✓ verify(42, 12345, C) = {} (correct opening)", is_valid);

    // Verify with wrong value
    let wrong_v = Fr::from(99u64);
    let is_invalid = verify(&wrong_v, &r, &c);
    assert!(!is_invalid);
    println!(
        "✓ verify(99, 12345, C) = {} (wrong value rejected)",
        is_invalid
    );
}

pub fn explore_hiding() {
    println!("\n=== Hiding Property ===\n");
    println!("Same value (v=42), three different blinding factors:\n");
    let v = Fr::from(42u64);
    let r1 = Fr::from(111u64);
    let r2 = Fr::from(222u64);
    let r3 = Fr::from(333u64);
    let c1 = commit(&v, &r1);
    let c2 = commit(&v, &r2);
    let c3 = commit(&v, &r3);
    println!("C1 (r=111): {:?}", c1);
    println!("C2 (r=222): {:?}", c2);
    println!("C3 (r=333): {:?}", c3);
    // All three are different — no way to tell they commit to the same value!
    assert_ne!(c1, c2);
    assert_ne!(c2, c3);
    assert_ne!(c1, c3);
    println!("\n✓ All 3 commitments are completely different");
    println!("  An observer sees 3 random curve points —");
    println!("  impossible to tell they all hide the same value!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commit_verify_roundtrip() {
        let v = Fr::from(100u64);
        let r = Fr::from(200u64);
        let c = commit(&v, &r);
        assert!(verify(&v, &r, &c));
    }

    #[test]
    fn test_wrong_blinding_fails() {
        let v = Fr::from(100u64);
        let r = Fr::from(200u64);
        let c = commit(&v, &r);
        let wrong_r = Fr::from(201u64);
        assert!(!verify(&v, &wrong_r, &c));
    }

    #[test]
    fn test_wrong_value_fails() {
        let v = Fr::from(100u64);
        let r = Fr::from(200u64);
        let c = commit(&v, &r);
        let wrong_v = Fr::from(101u64);
        assert!(!verify(&wrong_v, &r, &c));
    }
}
