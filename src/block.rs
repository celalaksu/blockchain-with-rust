use std::{fmt::{ self, Debug, Formatter }, vec};
use super::*; // imports everything

use crate::Hash;

pub struct Block {
    pub index: u32,
    pub timestamp: u128, // imported form lib.rs
    pub hash: Hash, // imported from lib.rs
    pub prev_block_hash: Hash,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub difficulty: u128,
}

impl Debug for Block {
    // Block tan türetilen nesneyi yazdırmak için kullanılan otomatik bir fonksiyondur.
    // block nesnesini çıktısı için format oluşturuyouruz.
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[{}] : {} at: {} with: {} nonce: {}",
            &self.index,
            hex::encode(&self.hash),
            &self.timestamp,
            &self.transactions.len(),
            &self.nonce,
        )
    }
}

impl Block {
    // struct örneğinin oluşmasını sağlar, constructor gibidir.
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: Hash,
        transactions: Vec<Transaction>,
        difficulty: u128,
    ) -> Self {
        Block { index, timestamp, hash: vec![0;32], prev_block_hash, nonce: 0, transactions, difficulty }
    }

    pub fn mine(&mut self){
        for nonce_attmept in 0..(u64::max_value()){
            self.nonce = nonce_attmept;
            let hash = self.hash();
            if check_difficulty(&hash, self.difficulty){
                self.hash = hash;
                return;
            }
        }
    }
}

impl Hashable for Block {

    // Blocktaki verilerin hepsi u8 türüne çevrilmektedir.
    // lib.rs tanımlanan fonksiyonlar bunun içindir.
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(self.transactions
        .iter().flat_map(|transaction| transaction.bytes()).collect::<Vec<u8>>());
        bytes.extend(&u128_bytes(&self.difficulty));

        bytes
    }
}

pub fn check_difficulty(hash: &Hash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash)
}
