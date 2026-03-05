# 🔐 crypto-primitives-rust

Building cryptographic primitives from scratch in Rust — ECC, BLS signatures, pairings, and zero-knowledge proof building blocks. **Strictly for learning.**

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

### 📁 [`bls/`](bls/) — BLS Signatures & Pairings

Exploring **pairing-based cryptography** using the [arkworks](https://github.com/arkworks-rs) library for the BLS12-381 curve. The focus is on understanding *how pairings work* and *why BLS signatures are special*.

#### ✅ Part 1 — Exploring the Groups ([`groups.rs`](bls/src/groups.rs))

Hands-on exploration of G₁, G₂, and the scalar field Fr.

| Function | What it demonstrates |
|----------|---------------------|
| `explore_generators()` | G₁ and G₂ generator points, group order |
| `explore_scalar_multiplication()` | Core `k * P` operation, group law verification |
| `explore_random_scalars()` | Random key generation with `Fr::rand()` |
| `explore_identity()` | Point at infinity — the "zero" of the group |

#### ✅ Part 2 — Understanding the Pairing ([`pairings.rs`](bls/src/pairings.rs))

Exploring the bilinear map `e: G₁ × G₂ → Gₜ` and proving its properties.

| Function | What it demonstrates |
|----------|---------------------|
| `explore_basic_pairing()` | First pairing call — what Gₜ looks like |
| `verify_bilinearity()` | Proof that `e(aP, bQ) = e(P, Q)^(ab)` |
| `verify_non_degeneracy()` | Pairing produces meaningful, distinct outputs |
| `preview_bls_equation()` | Full BLS verification: `e(σ, G₂) == e(H(m), pk)` |

#### 🔲 Part 3 — BLS Sign & Verify (coming next)
#### 🔲 Part 4 — Signature Aggregation
#### 🔲 Part 5 — Full Demo

---

## Running

```bash
# Run ECC project
cd ecc && cargo test

# Run BLS project
cd bls && cargo run      # see all demonstrations
cd bls && cargo test     # run unit tests
```

---

## Learning Roadmap

```
ECC (from scratch) ✅
 └──▶ BLS Signatures (in progress) 🔧
       └──▶ KZG Polynomial Commitments
             └──▶ PLONK (ZK-SNARK)
 └──▶ Pedersen Commitments
 └──▶ Schnorr Signatures
       └──▶ Sigma Protocols
```

---

## License

MIT License
