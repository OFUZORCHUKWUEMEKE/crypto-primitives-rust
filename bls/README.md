# BLS Signatures

A step-by-step Rust implementation of BLS (Boneh-Lynn-Shacham) signatures on the BLS12-381 curve, built for learning.

## What are BLS Signatures?

BLS is a digital signature scheme with one superpower: **aggregation**.

- Multiple signatures from multiple signers collapse into **one single signature**
- One signature, one verification check — regardless of how many people signed

## How It Works

```
Private Key:  sk  (random scalar)
Public Key:   pk = sk · G₂       (point on G2 curve)
Signature:    σ  = sk · H(m)     (point on G1 curve)

Verify:       e(σ, G₂) == e(H(m), pk)   ← pairing check
```

Where `e(·, ·)` is a **bilinear pairing** — a special map that lets you move scalar multiplication between the two sides.

## Structure

| File | What it covers |
|---|---|
| `groups.rs` | BLS12-381 G1/G2 groups and scalar multiplication |
| `pairings.rs` | Bilinear pairings and the verification equation |
| `bls.rs` | `keygen`, `sign`, `verify` |
| `aggregate.rs` | Aggregating N signatures → 1 signature |

## Ethereum Use Case

Ethereum's Proof-of-Stake uses BLS signatures for **validator attestations**:

- ~1 million validators attest to each block
- Each validator signs the same message ("I agree block X is valid")
- Without BLS: 1,000,000 signatures × 96 bytes = **~96 MB per block**
- With BLS: 1,000,000 signatures → **1 signature × 96 bytes**

This is why Ethereum chose BLS12-381 as its validator signature scheme — it makes consensus **scalable**.

## Key Properties

- **Aggregation**: `σ₁ + σ₂ + σ₃ = one valid aggregate signature`
- **Small signatures**: 96 bytes per signature (G1 point)
- **Non-interactive**: Signers don't talk to each other to aggregate
- **Deterministic**: Same key + same message = same signature always

## Run

```bash
cargo run    # explore groups, pairings, signing, aggregation, and size comparison
cargo test   # run unit tests
```

## Dependencies

Uses [`ark-bls12-381`](https://docs.rs/ark-bls12-381) — the exact curve specified in [EIP-2537](https://eips.ethereum.org/EIPS/eip-2537) for Ethereum's consensus layer.
