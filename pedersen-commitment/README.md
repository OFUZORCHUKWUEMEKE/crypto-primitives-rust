# Pedersen Commitments

A step-by-step Rust implementation of Pedersen Commitment Schemes on the BLS12-381 curve, built for learning.

## What is a Pedersen Commitment?

A commitment scheme is like putting a value in a **locked box**:
- **Commit**: lock your value inside — nobody can see it (**hiding**)
- **Reveal**: open the box later — you can't swap the value (**binding**)

The Pedersen formula: `C = v·G + r·H`

| Symbol | Meaning |
|---|---|
| `v` | The secret value you're committing to |
| `r` | A random blinding factor (hides `v`) |
| `G` | Standard BLS12-381 G1 generator |
| `H` | Second generator with unknown discrete log |
| `C` | The commitment (a curve point) |

## Structure

| File | What it covers |
|---|---|
| `generator.rs` | Setting up G and H |
| `commit.rs` | `commit(v, r)` and `verify(v, r, C)` |
| `homomorphic.rs` | `C1 + C2 = Commit(v1+v2, r1+r2)` |

## Key Properties

- **Hiding**: Same value + different `r` → completely different commitments
- **Binding**: Can't find another `(v', r')` that opens to the same `C` (requires solving discrete log)
- **Homomorphic**: Add/subtract commitments without revealing the values inside

## Real-World Usage

This is the core primitive behind:
- **Monero** — hides transaction amounts
- **Mimblewimble** — confidential transactions
- **Bulletproofs** — range proofs
- **ZK-SNARKs** — as a building block for polynomial commitments

## Run

```bash
cargo run    # see all 3 parts in action
cargo test   # run all 7 unit tests
```

## Dependencies

Uses [`ark-bls12-381`](https://docs.rs/ark-bls12-381) — the same curve used in Ethereum's BLS signatures.
