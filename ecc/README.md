# 🔑 ECC from Scratch

Elliptic Curve Cryptography implemented from the ground up in Rust — **no external crypto libraries**. Built to understand the math behind digital signatures.

> ⚠️ **Educational only. Not for production use.**

## What's Inside

| Module | What it does |
|--------|-------------|
| [`field.rs`](src/field.rs) | Finite field arithmetic (Fₚ) — modular add, sub, mul, exp, inverse |
| [`point.rs`](src/point.rs) | Elliptic curve points — addition, doubling, point at infinity |
| [`scalar.rs`](src/scalar.rs) | Scalar multiplication via double-and-add |
| [`ecdsa.rs`](src/ecdsa.rs) | Toy ECDSA — keygen, signing, verification |

## Run

```bash
cargo test
```
