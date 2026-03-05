use ark_bls12_381::{Bls12_381, Fr, G1Projective, G2Projective};
use ark_ec::{PrimeGroup, pairing::Pairing};
use ark_ff::Field;
use ark_std::UniformRand;
use sha2::{Digest, Sha256}; // Hashing

// In BLS:
// Private Key = Scalar (Fr)
// Public Key  = Point in G2
// Signature   = Point in G1
pub type PrivateKey = Fr;
pub type PublicKey = G2Projective;
pub type Signature = G1Projective;

pub fn keygen() -> (PrivateKey, PublicKey) {
    let mut rng = ark_std::test_rng();

    // sk = random scalar
    let sk = PrivateKey::rand(&mut rng);

    // pk = sk * G2_generator
    let pk = PublicKey::generator() * sk;

    (sk, pk)
}

pub fn hash_message_to_g1(message: &[u8]) -> G1Projective {
    // 1. Hash the message bytes using SHA-256
    let mut hasher = Sha256::new();
    hasher.update(message);

    let hash_result = hasher.finalize();
    // 2. Convert the 32-byte hash into a field element (scalar in Fr)
    // We use from_le_bytes_mod_order to safely reduce the 256-bit hash mod r.
    use ark_ff::PrimeField;
    let scalar = Fr::from_le_bytes_mod_order(&hash_result);
    // 3. Multiply the G1 generator by this scalar to get a pseudo-random curve point
    G1Projective::generator() * scalar
}

pub fn sign(sk: &PrivateKey, message: &[u8]) -> Signature {
    // 1. Hash message to a G1 point: H(m)
    let hm = hash_message_to_g1(message);
    // 2. Signature is just the private key times that point: σ = sk * H(m)
    hm * sk
}

pub fn verify(pk: &PublicKey, message: &[u8], sig: &Signature) -> bool {
    // 1. Hash the message back to the same G1 point: H(m)
    let hm = hash_message_to_g1(message);
    // 2. The pairing check: e(σ, G₂) == e(H(m), pk)
    //
    // Remember why this works (from Part 2):
    // e(σ, G₂) = e(sk * H(m), G₂) = e(H(m), G₂)^sk = e(H(m), sk * G₂) = e(H(m), pk)
    let lhs = Bls12_381::pairing(*sig, G2Projective::generator());
    let rhs = Bls12_381::pairing(hm, *pk);
    lhs == rhs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_and_verify() {
        let (sk, pk) = keygen();
        let message = b"Hello,BLS";

        let sig = sign(&sk, message);

        assert!(verify(&pk, message, &sig), "Signature should be valid");
    }

    #[test]
    fn test_tampered_message_fails() {
        let (sk, pk) = keygen();
        let msg1 = b"Pay Alice $100";
        let msg2 = b"Pay Mallory $100";

        let sig = sign(&sk, msg1);
        // Try to verify sig1 against msg2
        assert!(!verify(&pk, msg2, &sig), "Tampered message should fail");
    }

    #[test]
    fn test_wrong_public_key_fails() {
        // Use explicit different scalars instead of keygen(),
        // because test_rng() is deterministic and would give the same key twice.
        let sk_alice = Fr::from(12345u64);
        let pk_alice = G2Projective::generator() * sk_alice;

        let sk_eve = Fr::from(99999u64);
        let pk_eve = G2Projective::generator() * sk_eve;

        let message = b"Alice's authentic message";
        // Alice signs the message
        let sig = sign(&sk_alice, message);
        // Eve tries to claim she signed it using her public key
        assert!(
            !verify(&pk_eve, message, &sig),
            "Wrong public key should fail"
        );
    }
}
