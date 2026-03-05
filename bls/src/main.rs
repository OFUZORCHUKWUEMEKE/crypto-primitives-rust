mod aggregate;
mod bls;
mod groups;
mod pairings;

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
}
