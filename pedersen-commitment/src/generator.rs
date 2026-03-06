use ark_bls12_381::{Fr, G1Projective};
use ark_ec::PrimeGroup;
use ark_ff::PrimeField;
use sha2::{Digest, Sha256};

/// Creates generator G — the standard BLS12-381 G1 generator.
///
/// This is a well-known, fixed point on the curve.
/// Everyone uses the same G — it's part of the curve specification.
pub fn generator_g() -> G1Projective {
    G1Projective::generator()
}

pub fn generator_h() -> G1Projective {
    // 1. Hash a nothing-up-my-sleeve string
    let mut hasher = Sha256::new();
    hasher.update(b"pedersen_h_generator");
    let hash_bytes = hasher.finalize();

    // 2. Convert hash output to a scalar (element of Fr)
    //    Fr::from_le_bytes_mod_order takes arbitrary bytes and reduces mod r
    let h_scalar = Fr::from_le_bytes_mod_order(&hash_bytes);

    // 3. Multiply G by this scalar to get H
    //    H = h_scalar * G
    //    Nobody can reverse SHA-256 to find h_scalar from H,
    //    so the discrete log relationship is effectively unknown.
    let g = G1Projective::generator();
    g * h_scalar
}

pub fn explore_generators() {
    println!("=== Generator Setup ===\n");
    let g = generator_g();
    let h = generator_h();
    println!("G (standard generator): {:?}", g);
    println!("\nH (derived via hash):    {:?}", h);

    // Verify: G and H are different points
    assert_ne!(g, h, "G and H must be different points!");
    println!("\n✓ G ≠ H (they are different points)");
    // Verify: Neither is the identity (point at infinity)
    let identity = G1Projective::default();
    assert_ne!(g, identity, "G must not be the identity!");
    assert_ne!(h, identity, "H must not be the identity!");
    println!("✓ Neither G nor H is the identity point");

    // Verify: H is deterministic (same hash → same point every time)
    let h2 = generator_h();
    assert_eq!(h, h2, "H must be deterministic!");
    println!("✓ H is deterministic (same result every call)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generators_are_different() {
        let g = generator_g();
        let h = generator_h();
        assert_ne!(g, h);
    }

    #[test]
    fn test_generators_not_identity() {
        let g = generator_g();
        let h = generator_h();
        let identity = G1Projective::default();
        assert_ne!(g, identity);
        assert_ne!(h, identity);
    }
}
