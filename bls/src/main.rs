mod aggregate;
mod bls;
mod groups;
mod pairings;

use ark_bls12_381::{Fr, G2Projective};
use ark_ec::PrimeGroup;
use ark_serialize::CanonicalSerialize;

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║   Part 1: Exploring BLS12-381 Groups ║");
    println!("╚══════════════════════════════════════╝\n");

    groups::explore_generators();
    println!();

    groups::explore_scalar_multiplication();
    println!();

    groups::explore_random_scalars();
    println!();

    groups::explore_identity();
    println!();

    println!("\n╔══════════════════════════════════════╗");
    println!("║   Part 2: Exploring Pairings         ║");
    println!("╚══════════════════════════════════════╝\n");

    pairings::explore_basic_pairing();
    println!();

    pairings::verify_bilinearity();
    println!();

    pairings::verify_non_degeneracy();
    println!();

    pairings::preview_bls_equation();
    println!();

    println!("\n╔══════════════════════════════════════╗");
    println!("║   Part 3: BLS Sign and Verify        ║");
    println!("╚══════════════════════════════════════╝\n");
    let message = b"Hello, zero-knowledge world!";
    println!(
        "Message to sign: '{:?}'",
        std::str::from_utf8(message).unwrap()
    );
    // 1. Keygen
    let (sk, pk) = bls::keygen();
    println!("Keys generated!");
    // 2. Sign
    let sig = bls::sign(&sk, message);
    println!("Message signed!");
    // 3. Verify
    let is_valid = bls::verify(&pk, message, &sig);
    println!("Signature valid? {}", is_valid);

    // ═══════════════════════════════════════════
    //  Part 4: Signature Aggregation
    // ═══════════════════════════════════════════

    let consensus_msg = b"Block #1337 is valid";
    println!(
        "Message: \"{}\"",
        std::str::from_utf8(consensus_msg).unwrap()
    );

    // Create 3 validators
    let sk1 = Fr::from(1001u64);
    let pk1 = G2Projective::generator() * sk1;

    let sk2 = Fr::from(2002u64);
    let pk2 = G2Projective::generator() * sk2;

    let sk3 = Fr::from(3003u64);
    let pk3 = G2Projective::generator() * sk3;
    println!("✓ 3 validators created");

    // Each validator signs
    let sig1 = bls::sign(&sk1, consensus_msg);
    let sig2 = bls::sign(&sk2, consensus_msg);
    let sig3 = bls::sign(&sk3, consensus_msg);
    println!("✓ Each validator signed independently");

    // Verify each individually (3 checks = 6 pairings)
    assert!(bls::verify(&pk1, consensus_msg, &sig1));
    assert!(bls::verify(&pk2, consensus_msg, &sig2));
    assert!(bls::verify(&pk3, consensus_msg, &sig3));
    println!("✓ All 3 individual signatures verified (6 pairings)");

    // Aggregate into one signature (just point addition!)
    let agg_sig = aggregate::aggregate_signatures(&[sig1, sig2, sig3]);
    println!("✓ Aggregated 3 signatures → 1 signature");

    // Verify aggregate (1 check = 2 pairings)
    let agg_valid = aggregate::aggregate_verify(&[pk1, pk2, pk3], consensus_msg, &agg_sig);

    println!(
        "✓ Aggregate signature valid? {} (only 2 pairings!)",
        agg_valid
    );
    // ═══════════════════════════════════════════
    //  Part 5: Size Comparison
    // ═══════════════════════════════════════════
    println!("\n╔══════════════════════════════════════╗");
    println!("║   Part 5: The Power of Aggregation    ║");
    println!("╚══════════════════════════════════════╝\n");

    // Measure signature sizes
    let mut sig_bytes = Vec::new();
    sig1.serialize_compressed(&mut sig_bytes).unwrap();
    let single_sig_size = sig_bytes.len();
    let mut agg_sig_bytes = Vec::new();
    agg_sig.serialize_compressed(&mut agg_sig_bytes).unwrap();
    let agg_sig_size = agg_sig_bytes.len();
    println!("Single signature size:      {} bytes", single_sig_size);
    println!("3 separate signatures:      {} bytes", single_sig_size * 3);
    println!("1 aggregated signature:     {} bytes", agg_sig_size);
    println!(
        "Space saved:                {}%",
        100 - (agg_sig_size * 100) / (single_sig_size * 3)
    );
    println!("\n══════════════════════════════════════");
    println!("  Individual:  3 sigs × {} bytes = {} bytes, 6 pairings",
        single_sig_size, single_sig_size * 3);
    println!("  Aggregated:  1 sig  × {} bytes = {} bytes, 2 pairings",
        agg_sig_size, agg_sig_size);
    println!("══════════════════════════════════════");
    println!("\n🎉 BLS Signatures complete!");
    println!("   You now understand:");
    println!("   • Elliptic curve groups (G₁, G₂)");
    println!("   • Bilinear pairings");
    println!("   • BLS sign & verify");
    println!("   • Signature aggregation");
}
