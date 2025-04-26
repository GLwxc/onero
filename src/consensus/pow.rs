use crate::transaction::{Transaction,calculate_hash};
use crate::block::Block;
use crate::blockchain::Blockchain;
use super::Consensus; // import the trait

pub struct ProofOfWork;

impl Consensus for ProofOfWork {
    fn mine_block(&self, transactions: &Vec<Transaction>, prev_hash: &String, blockchain: &Blockchain) -> Block {
        // Implement PoW logic here
        let mut nonce: u32 = 0;

        // loop while we did not find a valid nonce.
        loop {
            let hash = calculate_hash(&nonce, transactions, prev_hash);

            if hash.starts_with(&"0".repeat(blockchain.difficulty as usize)) {
                return Block {
                    index: blockchain.chain.back().unwrap().index + 1, // Set appropriate values
                    timestamp: 0, // Set appropriate values
                    transactions: transactions.to_vec(),
                    prev_hash: prev_hash.to_string(),
                    hash: hash,
                    nonce,
                };
            }

            nonce += 1;
        }

    }
    
    fn validate_block(&self, block: &Block, prev_block: &Block, blockchain: &Blockchain) -> bool {
        // Recalculate the hash to verify it matches the block's hash
        let hash = calculate_hash(&block.nonce, &block.transactions, &block.prev_hash);

        // Check that the hash is valid for the current difficulty level
        let hash_is_valid = hash.starts_with(&"0".repeat(blockchain.difficulty as usize));

        // Validate that the block index is one more than the previous block's index
        let index_is_valid = block.index == prev_block.index + 1;

        // Validate that the previous hash in the block matches the hash of the previous block
        let prev_hash_is_valid = block.prev_hash == prev_block.hash;

        // Validate transactions to prevent double spending
        let transactions_are_valid = self.validate_transactions(&block.transactions, blockchain);

        hash_is_valid && index_is_valid && prev_hash_is_valid && transactions_are_valid
    }  

    fn validate_transactions(&self, transactions: &[Transaction], blockchain: &Blockchain) -> bool {
        // Implement logic to check each transaction
        // For example, you can iterate over the transactions and check the before and after balance for each public key
        // Ensure that the balance is never negative, indicating no double spending occurred
        for transaction in transactions {
            // Example check (implement the actual logic based on your blockchain's design)
            if !self.is_transaction_valid(transaction, blockchain) {
                return false;
            }
        }

        true
    }
    fn is_transaction_valid(&self, transaction: &Transaction, blockchain: &Blockchain) -> bool {
        // Implement the logic to validate a single transaction
        // This could include checking the sender's balance, verifying digital signatures, etc.
        // For now, this is just a placeholder function
        true
    }

}
