use super::*;
pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;

    fn hash(&self) -> Hash {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes()) //uses cryptohash crate to convert bytes to hash using sha256

    }
}