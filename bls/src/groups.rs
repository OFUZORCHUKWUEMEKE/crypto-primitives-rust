use ark_bls12_381::{Fr, G1Projective, G2Projective};
use ark_ec::PrimeGroup;
use ark_ff::PrimeField;
use ark_std::UniformRand;

// f

pub fn explore_generators() {
    // The generator is a fixed , well known point on the curve
    // Every other point in the group can be reached by  multiplying the the generator by some scalar: P = scalar * G

    let g1 = G1Projective::generator();
    let g2 = G2Projective::generator();

    println!("=== Generators ===");
    println!("G1 Generator: {:?}", g1);
    println!("G2 Generator: {:?}", g2);

    // Both groups have the same order (number of elements).
    // This is the prime number r, roughly 2^255.
    // Any scalar used for keys/signing is an element of Fr (integers mod r).
    println!("\nScalar field modulus (group order): {:?}", Fr::MODULUS);
}

pub fn explore_scalar_multiplication() {
    let g1 = G1Projective::generator();
    let g2 = G2Projective::generator();

    // Create a known scalar. In real BLS this would be your private key.
    let scalar = Fr::from(42u64);

    // Multiply the generators by the scalar.
    // This is the core operation in all ECC:
    //   - Public key = sk * G2
    //   - Signature  = sk * H(m) in G1

    let p1 = g1 * scalar;
    let p2 = g2 * scalar;
    println!("=== Scalar Multiplication ===");
    println!("42 * G1 = {:?}", p1);
    println!("42 * G2 = {:?}", p2);

    // Verify: 2 * G1 + 3 * G1 should equal 5 * G1
    // This proves the group law works as expected.
    let two_g1 = g1 * Fr::from(2u64);
    let three_g1 = g1 * Fr::from(3u64);
    let five_g1 = g1 * Fr::from(5u64);

    assert_eq!(two_g1 + three_g1, five_g1);
    println!("\n✓ Verified: 2*G1 + 3*G1 == 5*G1 (group law works!)");
}

pub fn explore_random_scalars() {
    // Create a random number generator.
    // test_rng() gives a deterministic RNG — same output every run.
    // Good for learning and testing.
    let mut rng = ark_std::test_rng();

    // Sample a random scalar — this is how you'd generate a private key.
    let random_scalar = Fr::rand(&mut rng);
    println!("=== Random Elements ===");
    println!("Random scalar (private key): {:?}", random_scalar);

    // Multiply generators by the random scalar — this is keygen!
    let random_g1 = G1Projective::generator() * random_scalar;
    let random_g2 = G2Projective::generator() * random_scalar;

    println!("Random G1: {:?}", random_g1);
    println!("Random G2: {:?}", random_g2);
}

pub fn explore_identity() {
    let g1 = G1Projective::generator();
    let identity = G1Projective::default(); // The identity (point at infinity)
    println!("=== Identity Element ===");
    println!("Identity (zero point): {:?}", identity);

    // Any point + identity = that point (identity is the additive zero)
    assert_eq!(g1 + identity, g1);
    println!("✓ G1 + identity == G1");

    // A point minus itself = identity
    assert_eq!(g1 - g1, identity);
    println!("✓ G1 - G1 == identity");

    // Multiplying by zero gives the identity
    assert_eq!(g1 * Fr::from(0u64), identity);
    println!("✓ 0 * G1 == identity");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_law() {
        // a*G + b*G should equal (a+b)*G
        let g1 = G1Projective::generator();
        let a = Fr::from(123u64);
        let b = Fr::from(456u64);
        let sum = a + b; // scalar addition in Fr
        assert_eq!(g1 * a + g1 * b, g1 * sum);
    }

    #[test]
    fn test_identity() {
        let g1 = G1Projective::generator();
        let identity = G1Projective::default();
        assert_eq!(g1 + identity, g1);
        assert_eq!(g1 - g1, identity);
    }
}
