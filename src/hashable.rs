use crate::Hash;

// use super::*;
pub trait Hashable {
    fn bytes(&self) -> Hash;

    fn hash(&self) -> Vec<u8>{
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}