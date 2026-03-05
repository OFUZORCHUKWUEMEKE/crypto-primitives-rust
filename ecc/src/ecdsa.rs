use crate::point::Point;

/// ECDSA Signature containing components R and S
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signature {
    pub r: u128,
    pub s: u128,
}

/// A simplified ECDSA implementation for educational purposes.
pub struct Ecdsa<'a> {
    pub generator: Point,
    pub order: u128,
    /// We simulate a hash function returning a u128 directly for simplicity
    pub hash_fn: &'a dyn Fn(&[u8]) -> u128,
}

impl<'a> Ecdsa<'a> {
    pub fn new(generator: Point, order: u128, hash_fn: &'a dyn Fn(&[u8]) -> u128) -> Self {
        Self {
            generator,
            order,
            hash_fn,
        }
    }

    /// Generate a Keypair from a given secret integer.
    /// (Private Key, Public Key)
    pub fn generate_keypair(&self, secret_key: u128) -> (u128, Point) {
        assert!(
            secret_key > 0 && secret_key < self.order,
            "Secret key must be in range (0, order)"
        );
        let public_key = secret_key * self.generator;
        (secret_key, public_key)
    }

    /// Modular inverse using the Extended Euclidean Algorithm (works for non-primes)
    fn mod_inverse(num: u128, modulo: u128) -> Option<u128> {
        let mut t = 0i128;
        let mut newt = 1i128;
        let mut r = modulo as i128;
        let mut newr = num as i128;

        while newr != 0 {
            let quotient = r / newr;

            let temp_t = t - quotient * newt;
            t = newt;
            newt = temp_t;

            let temp_r = r - quotient * newr;
            r = newr;
            newr = temp_r;
        }

        if r > 1 {
            return None; // Not coprime
        }

        if t < 0 {
            t += modulo as i128;
        }

        Some(t as u128)
    }

    /// Signs a message using the given private key and a random nonce `k`.
    ///
    /// Note: In a real implementation this `k` must be strictly randomly generated per-message
    /// and NEVER reused. Reusing `k` compromises the private key immediately.
    pub fn sign(&self, message: &[u8], private_key: u128, nonce: u128) -> Signature {
        assert!(
            nonce > 0 && nonce < self.order,
            "Nonce must be in range (0, order)"
        );

        // 1. Calculate the hash of the message (z)
        let z = (self.hash_fn)(message);

        // 2. Calculate the random point R = k * G
        let r_point = nonce * self.generator;

        // 3. Extract the x-coordinate of R, modulo the order (r)
        // If r == 0, the signature is invalid (probability is infinitesimally small)
        let r = r_point.x.expect("R cannot be infinity").num % self.order;
        assert_ne!(r, 0, "r cannot be 0 in ECDSA (choose different nonce)");

        // 4. Calculate s = k^-1 * (z + r * private_key) mod order
        // Note: The arithmetic for 's' happens in the scalar field F_order.
        // We use our custom mod_inverse because the order is not necessarily prime.
        let k_inv =
            Self::mod_inverse(nonce, self.order).expect("Nonce k must be coprime with order");
        let r_term = (r * private_key) % self.order;
        let z_plus_r_term = (z + r_term) % self.order;
        let s = (k_inv * z_plus_r_term) % self.order;

        assert_ne!(s, 0, "s cannot be 0 in ECDSA (choose different nonce)");

        Signature { r, s }
    }

    /// Verifies a signature against the public key and message.
    pub fn verify(&self, message: &[u8], signature: Signature, public_key: Point) -> bool {
        let r = signature.r;
        let s = signature.s;

        // Signature components must be within the valid scalar field range
        if r == 0 || r >= self.order || s == 0 || s >= self.order {
            return false;
        }

        // 1. Calculate the hash of the message (z)
        let z = (self.hash_fn)(message);

        // 2. Calculate w = s^-1 mod order
        let w = match Self::mod_inverse(s, self.order) {
            Some(inv) => inv,
            None => return false, // Signature 's' is invalid if it has no inverse
        };

        // 3. Calculate u1 = z * w mod order
        let u1 = (z * w) % self.order;

        // 4. Calculate u2 = r * w mod order
        let u2 = (r * w) % self.order;

        // 5. Calculate validation point P = u1 * G + u2 * PublicKey
        let u1_g = u1 * self.generator;
        let u2_pub = u2 * public_key;
        let p = u1_g + u2_pub;

        if p.x.is_none() {
            return false;
        }

        // 6. Verification succeeds if x-coordinate of P modulo order == r
        let p_x = p.x.unwrap().num % self.order;

        p_x == r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::FieldElement;

    #[test]
    fn test_ecdsa_sign_verify() {
        // secp256k1 toy parameters
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);

        let g_x = FieldElement::new(47, prime);
        let g_y = FieldElement::new(71, prime);
        let generator = Point::new(Some(g_x), Some(g_y), a, b);
        let order = 21; // The order of group generated by point (47, 71) on this curve

        // Dummy hash function returning first byte of message
        let mock_hash = |data: &[u8]| -> u128 { data[0] as u128 };
        let ecdsa = Ecdsa::new(generator, order, &mock_hash);

        let private_key = 1; // Secret integer
        let (_, public_key) = ecdsa.generate_keypair(private_key);

        let message = b"A"; // Just a byte array
        let nonce = 2; // Random ephemeral key "k"

        println!("pub: {:?}", public_key);

        let sig = ecdsa.sign(message, private_key, nonce);

        // Verify successful validation
        assert!(ecdsa.verify(message, sig, public_key));

        // Verify a tampered signature fails
        let bad_sig = Signature {
            r: sig.r,
            s: sig.s + 1,
        };
        assert!(!ecdsa.verify(message, bad_sig, public_key));

        // Verify a tampered message fails
        let bad_message = b"B";
        assert!(!ecdsa.verify(bad_message, sig, public_key));
    }
}
