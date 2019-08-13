use num::BigUint;
use serde::{Deserialize, Serialize};

/// A proof of posession of the private key.
///
/// A non-interactive zero-knowledge proof of knowledge of a private
/// key `s` corresponding to a public key `h`.
#[derive(Serialize, Deserialize)]
pub struct Proof {
    /// The challenge `c` that is produced by hashing relevent
    /// parameters, including the original public key `h` and the
    /// one-time public key `k`.
    #[serde(deserialize_with = "crate::deserialize::biguint")]
    challenge: BigUint,

    /// The one-use public key `k = gʳ` generated from the random
    /// one-use private key `r`. This acts as a committment to `r`.
    #[serde(deserialize_with = "crate::deserialize::biguint")]
    committment: BigUint,

    /// The response `u = r + c s mod (p - 1)` to the challenge, where
    /// `r` is the one-time private key corresponding to the one-time
    /// public key `k`, and `s` is the private-key corresponding to
    /// the original public key `h`.
    #[serde(deserialize_with = "crate::deserialize::biguint")]
    response: BigUint,
}