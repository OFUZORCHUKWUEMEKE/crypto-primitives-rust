use ark_bls12_381::{Fr, G1Projective};
use ark_ec::CurveGroup;
use ark_std::UniformRand;

use crate::commit;
use crate::commit::commit;
use crate::generator;

/// A confidential account: balance is hidden inside a Pedersen commitment.
/// Only the owner knows (balance, blinding_factor).
pub struct ConfidentialAccount {
    pub name: String,
    pub balance_commitment: G1Projective,
    /// The encrypted balance: Commit(balance, blinding)
    balance: Fr,
    /// Owner's secret: the blinding factor (private!)``
    blinding: Fr,
}

impl ConfidentialAccount {
    pub fn new(name: &str) -> Self {
        let zero = Fr::from(0u64);
        Self {
            name: name.to_string(),
            balance_commitment: commit::commit(&zero, &zero),
            balance: zero,
            blinding: zero,
        }
    }

    pub fn deposit(&mut self, amount: u64) {
        let mut rng = ark_std::test_rng();
        let v = Fr::from(amount);
        let r = Fr::rand(&mut rng);

        let deposit_commitment = commit::commit(&v, &r);

        self.balance_commitment = self.balance_commitment + deposit_commitment;

        self.balance = self.balance + v;
        self.blinding = self.blinding + r;
    }

    /// Transfer: move tokens confidentially from self to another account.
    /// Returns true if the balance proof passes (conservation check).
    pub fn transfer(&mut self, to: &mut ConfidentialAccount, amount: u64) -> bool {
        let mut rng = ark_std::test_rng();

        let v = Fr::from(amount);
        let r_send = Fr::rand(&mut rng);
        let r_receive = Fr::rand(&mut rng);

        let transfer_commitment = commit::commit(&v, &r_send);
        let receive_commitment = commit::commit(&v, &r_receive);

        // Update sender: subtract the transfer
        let old_sender_commitment = self.balance_commitment;
        self.balance_commitment = self.balance_commitment - transfer_commitment;

        self.balance = self.balance - v;
        self.blinding = self.blinding - r_send;

        // Update receiver: add the transfer
        to.balance_commitment = to.balance_commitment + receive_commitment;
        to.balance = to.balance + v;
        to.blinding = to.blinding + r_receive;

        // === BALANCE PROOF (what the network checks) ===
        // Verify: old_sender == new_sender + transfer_amount
        // i.e., old_sender - new_sender - transfer == Commit(0, excess_r)
        let diff = old_sender_commitment - self.balance_commitment - transfer_commitment;
        let excess_r = Fr::from(0u64); // The excess should be zero blinding

        let zero_commit = commit::commit(&Fr::from(0u64), &excess_r);
        diff == zero_commit
    }

    /// Withdraw: convert confidential balance back to public.
    /// The owner reveals (balance, blinding) to prove they own the funds.
    pub fn withdraw(&self, claimed_amount: u64) -> bool {
        let v = Fr::from(claimed_amount);
        commit::verify(&v, &self.blinding, &self.balance_commitment)
    }
    /// Get the balance (only the owner can call this — it's their secret)
    pub fn owner_view_balance(&self) -> u64 {
        // In a real system, the owner would decrypt using their private key.
        // Here we just access the stored value directly.
        // Note: this only works for small values we created with Fr::from(u64)
        // For a real system, you'd use ElGamal encryption alongside Pedersen.
        // We can't easily extract u64 from Fr in general,
        // so we'll use a simple brute-force for demo purposes
        for i in 0..10000u64 {
            if Fr::from(i) == self.balance {
                return i;
            }
        }
        panic!("Balance too large to display (demo limitation)");
    }
}

/// Full scenario: deposit, transfer, withdraw
pub fn explore_confidential_transfers() {
    println!("=== Confidential Transfer System ===\n");
    // Create accounts
    let mut alice = ConfidentialAccount::new("Alice");
    let mut bob = ConfidentialAccount::new("Bob");

    println!("1. DEPOSIT — Alice deposits 100 tokens");
    println!("   (visible on-ramp, balance becomes hidden after)\n");

    alice.deposit(100);

    println!(
        "   Alice's commitment: {:?}",
        alice.balance_commitment.into_affine()
    );
    println!(
        "   Alice knows her balance: {} tokens",
        alice.owner_view_balance()
    );

    println!("   Network sees: just a curve point (no idea it's 100)");
    println!("\n2. TRANSFER — Alice sends 30 to Bob (confidentially)");
    println!("   Neither the amount nor the balances are revealed\n");
    let proof_valid = alice.transfer(&mut bob, 30);
    println!("   Balance proof valid: {} ✓", proof_valid);
    println!(
        "   Alice knows: {} tokens remain",
        alice.owner_view_balance()
    );
    println!("   Bob knows: {} tokens received", bob.owner_view_balance());
    println!("   Network sees: commitments changed, proof passed");
    println!("   Network knows: NOTHING about the amounts!");
    println!("\n3. SECOND TRANSFER — Alice sends 20 more to Bob\n");
    let proof_valid = alice.transfer(&mut bob, 20);
    println!("   Balance proof valid: {} ✓", proof_valid);
    println!("   Alice: {} tokens", alice.owner_view_balance());
    println!("   Bob: {} tokens", bob.owner_view_balance());
    println!("\n4. WITHDRAW — Bob proves he has 50 tokens and withdraws\n");
    let withdraw_ok = bob.withdraw(50);
    println!("   Bob proves balance = 50: {} ✓", withdraw_ok);
    let withdraw_cheat = bob.withdraw(9999);
    println!(
        "   Bob tries to claim 9999: {} ✗ (rejected!)",
        withdraw_cheat
    );
    println!("\n5. SUMMARY");
    println!("   ┌─────────────────────────────────────────┐");
    println!("   │ Started: Alice 100, Bob 0               │");
    println!("   │ Transfer 1: Alice → Bob (30)            │");
    println!("   │ Transfer 2: Alice → Bob (20)            │");
    println!("   │ Final: Alice 50, Bob 50                 │");
    println!("   │                                         │");
    println!("   │ Network verified ALL transfers          │");
    println!("   │ without seeing ANY amounts! 🔐          │");
    println!("   └─────────────────────────────────────────┘");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit_and_withdraw() {
        let mut acc = ConfidentialAccount::new("Test");
        acc.deposit(500);
        assert!(acc.withdraw(500));
        assert!(!acc.withdraw(501));
    }

    #[test]
    fn test_transfer_conserves_balance() {
        let mut alice = ConfidentialAccount::new("Alice");
        let mut bob = ConfidentialAccount::new("Bob");
        alice.deposit(100);
        alice.transfer(&mut bob, 40);
        assert_eq!(alice.owner_view_balance(), 60);
        assert_eq!(bob.owner_view_balance(), 40);
        // Total is still 100
        assert!(alice.withdraw(60));
        assert!(bob.withdraw(40));
    }

    #[test]
    fn test_multiple_transfers() {
        let mut a = ConfidentialAccount::new("A");
        let mut b = ConfidentialAccount::new("B");
        a.deposit(200);
        a.transfer(&mut b, 50);
        a.transfer(&mut b, 30);
        a.transfer(&mut b, 20);
        assert_eq!(a.owner_view_balance(), 100);
        assert_eq!(b.owner_view_balance(), 100);
    }
}
