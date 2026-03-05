use crate::bls::{self, PublicKey, Signature};
use ark_bls12_381::{Bls12_381, Fr, G1Projective, G2Projective};
use ark_ec::{PrimeGroup, pairing::Pairing};
use ark_std::UniformRand;

pub fn aggregate_signatures(signatures: &[Signature]) -> Signature {
    // Aggregation is literally just adding all the G1 points together.
    // σ_agg = σ₁ + σ₂ + ... + σₙ
    let mut agg = G1Projective::default(); // start with identity (zero)
    for sig in signatures {
        agg = agg + sig;
    }
    agg
}

pub fn aggregate_public_keys(public_keys: &[PublicKey]) -> PublicKey {
    // pk_agg = pk₁ + pk₂ + ... + pkₙ
    let mut agg = G2Projective::default(); // start with identity
    for pk in public_keys {
        agg = agg + pk;
    }
    agg
}

pub fn aggregate_verify(public_keys: &[PublicKey], message: &[u8], agg_sig: &Signature) -> bool {
    // 1. Aggregate all the public keys
    let agg_pk = aggregate_public_keys(public_keys);

    // 2. Hash the message to G1
    let hm = bls::hash_message_to_g1(message);

    // 3. Single pairing check — same as normal verify but with aggregated values
    //    e(σ_agg, G₂) == e(H(m), pk_agg)
    let lhs = Bls12_381::pairing(*agg_sig, G2Projective::generator());
    let rhs = Bls12_381::pairing(hm, agg_pk);

    lhs == rhs
}
