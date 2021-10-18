use std::fmt::{self, Debug,Formatter};

use super::*; //imports files from the lib.rs

pub struct Block { //struct for a block, public so that it can be accessed from other modules
    pub index: u32,
    pub timestamp: u128,
    pub hash: Hash,
    pub prev_block_hash: Hash,
    pub nonce: u64,
    pub payload: String,
     
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[{}]: {} at: {} with: {},",
        &self.index,
        &hex::encode(&self.hash),
        &self.timestamp,
        &self.payload,

    
    
    )
    }
}
impl Block { //implement functions corresponds to block struct
    //implement a method for creating a  block
    pub fn new(index: u32, timestamp: u128, prev_block_hash: Hash,nonce: u64, payload: String) -> Self {
        Block {
            index,
            timestamp,
            hash:vec![0;32], //array initializer, writing 32 element long array of zeros
            prev_block_hash,
            nonce,
            payload,
        }
    }
}

impl Hashable for Block { //implement hashing function for a block that hashes the bytes(data) together
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![]; //declares a mutable bytes that is a vector array
        bytes.extend(&u32_bytes(&self.index)); //inputs index data into the byte at the end of the vector using the .extend method
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash); //adds the previous blockhash to the bytes to form a blockchain link
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(self.payload.as_bytes()); //gets the data the from the payload and turns it into bytes. no ampersand because as_bytes already returns a reference
        bytes

    }
}