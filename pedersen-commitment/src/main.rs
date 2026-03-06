mod commit;
mod generator;
mod homomorphic;
fn main() {
   
    generator::explore_generators();
    println!();

  
    commit::explore_commit_verify();
    println!();
    commit::explore_hiding();
    println!();

   
    homomorphic::explore_addition();
    println!();
    homomorphic::explore_subtraction();
    println!();

    println!("   • C1 + C2 = Commit(v1+v2, r1+r2) — add hidden values");
    println!("   • C1 - C2 = Commit(v1-v2, r1-r2) — subtract hidden values");
}
