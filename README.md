# 🔐 crypto-primitives-rust

Building cryptographic primitives from scratch in Rust — ECC, BLS signatures, pairings, Pedersen commitments, and zero-knowledge proof building blocks. **Strictly for learning.**

> ⚠️ **DO NOT USE IN PRODUCTION.** This code is educational only. It has not been audited, is not constant-time, and is not secure for any real-world application.

---

## Projects

### 📁 [`ecc/`](ecc/) — Elliptic Curve Cryptography from Scratch

A ground-up implementation of ECC **without any external crypto libraries**. Everything is built by hand to understand the math.

| Module | What it implements |
|--------|--------------------|
| [`field.rs`](ecc/src/field.rs) | Finite field arithmetic (Fₚ) — add, subtract, multiply, modular exponentiation, modular inverse |
| [`point.rs`](ecc/src/point.rs) | Elliptic curve points — point addition, point doubling, point at infinity |
| [`scalar.rs`](ecc/src/scalar.rs) | Scalar multiplication — double-and-add algorithm, O(log k) |
| [`ecdsa.rs`](ecc/src/ecdsa.rs) | Toy ECDSA — key generation, message signing, signature verification |

**Key concepts**: Finite fields, elliptic curve group law, discrete logarithm problem, digital signatures.

---

### 📁 [`bls/`](bls/) — BLS Signatures & Pairings ✅

Pairing-based cryptography using [arkworks](https://github.com/arkworks-rs) on BLS12-381.

| Part | File | What it covers |
|------|------|---------------|
| 1 | [`groups.rs`](bls/src/groups.rs) | G₁/G₂ generators, scalar multiplication, identity |
| 2 | [`pairings.rs`](bls/src/pairings.rs) | Bilinear pairing `e(P,Q)`, bilinearity, non-degeneracy |
| 3 | [`bls.rs`](bls/src/bls.rs) | `keygen`, `sign`, `verify` via `e(σ, G₂) == e(H(m), pk)` |
| 4–5 | [`aggregate.rs`](bls/src/aggregate.rs) | Aggregate N signatures → 1, size comparison |

**Ethereum context**: Ethereum PoS uses BLS12-381 for validator attestations — 1M signatures collapsed into one 96-byte signature.

---

### 📁 [`pedersen-commitment/`](pedersen-commitment/) — Pedersen Commitments ✅

Commit to a secret value without revealing it: `C = v·G + r·H`.

| Part | File | What it covers |
|------|------|---------------|
| 1 | [`generator.rs`](pedersen-commitment/src/generator.rs) | G (standard) and H (hash-derived), why independence matters |
| 2 | [`commit.rs`](pedersen-commitment/src/commit.rs) | `commit(v, r)`, `verify(v, r, C)`, hiding & binding demos |
| 3 | [`homomorphic.rs`](pedersen-commitment/src/homomorphic.rs) | `C1 + C2 = Commit(v1+v2, r1+r2)`, balance proofs |

**Real-world use**: Monero (hidden amounts), Mimblewimble, Bulletproofs, ZK-SNARKs.

---

## Running

```bash
# Run ECC project
cd ecc && cargo test

# Run BLS project
cd bls && cargo run      # see all demonstrations
cd bls && cargo test     # run unit tests

# Run Pedersen Commitment project
cd pedersen-commitment && cargo run # see demonstration
cd pedersen-commitment && cargo test # run unit tests
```

---

## Learning Roadmap

```
ECC (from scratch) ✅
 └──▶ BLS Signatures & Aggregation ✅
       └──▶ Pedersen Commitments ✅
             └──▶ Polynomial Commitments (KZG / IPA) 🔲
                   └──▶ ZK-SNARKs (PLONK / Groth16) 🔲
```

