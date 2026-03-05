# Elliptic Curve Cryptography (ECC) from Scratch

This project is a learning-focused implementation of Elliptic Curve Cryptography (ECC) written entirely from scratch in Rust, without using any external cryptography or math libraries.

The goal of this project is to demystify the math and mechanics behind modern cryptography algorithms like ECDSA by building them from the ground up, starting from modular arithmetic and scaling up to digital signatures.

## Important Note

⚠️ **DO NOT USE THIS IN PRODUCTION.** ⚠️
This codebase is strictly for educational purposes. It has not been audited, does not operate in constant time (and is therefore vulnerable to side-channel attacks), uses a small toy prime for demonstration purposes, and is not secure for any real-world application.

## Implementation Steps

This project is being built in a step-by-step manner:

### Step 1: Finite Field Arithmetic
Implemented `FieldElement` to represent numbers in a finite field $\mathbb{F}_p$.
- **Features:** Modular addition, subtraction, multiplication, exponentiation (using square-and-multiply), and modular inverse (using the Extended Euclidean Algorithm).
- **Location:** [`src/field.rs`](src/field.rs)

### Step 2: Elliptic Curve Points
Implemented `Point` to represent points on an elliptic curve $y^2 = x^3 + ax + b$ operating over the finite field.
- **Features:** Point validation, representation of the Point at Infinity, Algebraic Point Addition, and Algebraic Point Doubling.
- **Location:** [`src/point.rs`](src/point.rs)

### Step 3: Scalar Multiplication (Pending)
Will implement the Double-and-Add algorithm to multiply a curve point by a scalar integer $k$ ($k \cdot G$). Time complexity will be $O(\log k)$.

### Step 4: Toy ECDSA (Pending)
Will implement an educational version of the Elliptic Curve Digital Signature Algorithm (ECDSA), including:
- Key Generation (Private/Public pairs)
- Message Signing
- Signature Verification

## Running the Tests

The project includes unit tests for the core mathematical properties at every step. You can run them using standard Cargo commands:

```bash
cargo test
```
## License

MIT License
