mod groups;
mod pairings;
mod bls;

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
}
