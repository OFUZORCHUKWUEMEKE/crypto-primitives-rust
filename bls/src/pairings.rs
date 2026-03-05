use ark_bls12_381::{Bls12_381, Fr, G1Projective, G2Projective};
use ark_ec::{pairing::Pairing, PrimeGroup};
use ark_ff::Field;

pub fn explore_basic_pairing() {
    let g1 = G1Projective::generator();
    let g2 = G2Projective::generator();

    // THE PAIRING: takes a G1 point and a G2 point, outputs a Gt element.
    // We convert to affine first — the pairing function requires affine points.
    // .into() converts Projective → Affine automatically.
    let gt = Bls12_381::pairing(g1, g2);
}

pub fn verify_bilinearity() {
    let g1 = G1Projective::generator();
    let g2 = G2Projective::generator();

    let a = Fr::from(7u64);
    let b = Fr::from(13u64);
    // Compute the pairing three different ways that should all be equal:
    // Way 1: Move scalar 'a' into the first argument
    //   e(a*G1, b*G2)
    let left = Bls12_381::pairing(g1 * a, g2 * b);
    // Way 2: Move scalar 'b' into the second argument
    //   e(G1, ab*G2)
    let right_1 = Bls12_381::pairing(g1, g2 * (a * b));
    // Way 3: Compute e(G1, G2) first, then raise to the power ab
    //   e(G1, G2)^(ab)
    let base = Bls12_381::pairing(g1, g2);
    let ab = a * b;
    // In Gt, "scalar multiplication" is exponentiation.
    // We need to convert Fr scalar to a BigInt for pow().
    use ark_ff::PrimeField;
    let right_2 = base.0.pow(ab.into_bigint());

    println!("=== Bilinearity ===");
    println!("e(7*G1, 13*G2) = e(G1, G2)^(7*13) ?");
    assert_eq!(left, right_1);
    println!("✓ e(a*G1, b*G2) == e(G1, ab*G2)");
    assert_eq!(left.0, right_2);
    println!("✓ e(a*G1, b*G2) == e(G1, G2)^(ab)");
}

pub fn verify_non_degeneracy() {
    let g1 = G1Projective::generator();
    let g2 = G2Projective::generator();

    // Non-degeneracy: pairing of generators is not the identity element.
    // If e(G1, G2) = 1, the pairing would be useless — every output
    // would be 1, and you couldn't distinguish anything.
    let gt = Bls12_381::pairing(g1, g2);
    let gt_identity = <Bls12_381 as Pairing>::TargetField::ONE;

    println!("=== Non-Degeneracy ===");
    assert_ne!(gt.0, gt_identity);
    println!("✓ e(G1, G2) ≠ 1  (pairing is non-degenerate)");
    // Different inputs should give different outputs.
    let gt_2 = Bls12_381::pairing(g1 * Fr::from(2u64), g2);
    let gt_3 = Bls12_381::pairing(g1 * Fr::from(3u64), g2);
    assert_ne!(gt_2, gt_3);
    println!("✓ e(2*G1, G2) ≠ e(3*G1, G2)  (different inputs → different outputs)");
    // But pairing with the identity point DOES give the identity in Gt.
    let zero = G1Projective::default(); // point at infinity
    let gt_zero = Bls12_381::pairing(zero, g2);
    assert_eq!(gt_zero.0, gt_identity);
    println!("✓ e(identity, G2) == 1  (zero in → zero out)");
}

pub fn preview_bls_equation() {
    let g2 = G2Projective::generator();

    // Simulate a private key
    let sk = Fr::from(42u64);

    // Public key = sk * G2
    let pk = g2 * sk;

    // Pretend H(m) is some G1 point (
    let h_m = G1Projective::generator() * Fr::from(123u64); // fake hash

    // Signature = sk * H(m)
    let sigma = h_m * sk;
    // === THE BLS VERIFICATION EQUATION ===
    // Check: e(sigma, G2) == e(H(m), pk)
    //
    // Why it works:
    //   e(sigma, G2) = e(sk * H(m), G2)     ← substitute sigma = sk * H(m)
    //                = e(H(m), G2)^sk        ← bilinearity: pull sk out
    //                = e(H(m), sk * G2)      ← bilinearity: push sk to other side
    //                = e(H(m), pk)           ← substitute pk = sk * G2
    let lhs = Bls12_381::pairing(sigma, g2);
    let rhs = Bls12_381::pairing(h_m, pk);

    println!("=== BLS Verification Preview ===");
    assert_eq!(lhs, rhs);
    println!("✓ e(σ, G2) == e(H(m), pk)");
    println!("  BLS verification works! 🎉");

    // Now try with a WRONG signature (different sk)
    let wrong_sk = Fr::from(99u64);
    let wrong_sigma = h_m * wrong_sk;
    let wrong_lhs = Bls12_381::pairing(wrong_sigma, g2);
    assert_ne!(wrong_lhs, rhs);
    println!("✓ e(wrong_σ, G2) ≠ e(H(m), pk)");
    println!("  Wrong signature correctly rejected!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bilinearity() {
        let g1 = G1Projective::generator();
        let g2 = G2Projective::generator();

        let a = Fr::from(5u64);
        let b = Fr::from(11u64);

        // e(aG1, bG2) should equal e(abG1, G2)
        let lhs = Bls12_381::pairing(g1 * a, g2 * b);
        let rhs = Bls12_381::pairing(g1 * (a * b), g2);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_pairing_changes_with_input() {
        let g1 = G1Projective::generator();
        let g2 = G2Projective::generator();
        let e1 = Bls12_381::pairing(g1, g2);
        let e2 = Bls12_381::pairing(g1 * Fr::from(2u64), g2);
        assert_ne!(e1, e2);
    }

    #[test]
    fn test_bls_equation_holds() {
        let g2 = G2Projective::generator();
        let sk = Fr::from(777u64);
        let pk = g2 * sk;
        let h_m = G1Projective::generator() * Fr::from(999u64);
        let sigma = h_m * sk;
        assert_eq!(Bls12_381::pairing(sigma, g2), Bls12_381::pairing(h_m, pk));
    }
}
