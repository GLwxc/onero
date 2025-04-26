use crate::{Transaction};

#[derive(Debug, Clone)]// Define the Block struct
pub struct Block {
    pub index: u32,         // Block's position in the chain
    pub timestamp: u64,     // Time when the block was created
    pub transactions: Vec<Transaction>, // List of transactions in the block
    pub prev_hash: String,  // Hash of the previous block
    pub hash: String,       // Hash of the block itself
    pub nonce: u32,         // Nonce
}

//#[derive(Debug, Clone)]// Define the Transaction struct (assuming you have a separate transaction module)
//pub struct Transaction {
//    pub sender: String,
//    pub receiver: String,
//    pub amount: u32,
//}

impl Block {
    // Constructor method to create a new block
    pub fn new(index: u32, timestamp: u64, transactions: Vec<Transaction>, prev_hash: String, hash: String, nonce: u32) -> Self {
        Block {
            index,
            timestamp,
            transactions,
            prev_hash,
            hash,
            nonce,
        }
    }

}
