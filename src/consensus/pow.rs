use crate::transaction::{Transaction,calculate_hash};
use crate::block::Block;
use crate::blockchain::Blockchain;
use super::Consensus; // import the trait
use chrono::Utc;

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
                    timestamp: Utc::now().timestamp().try_into().unwrap(), // Set appropriate values
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

        let index_is_valid = block.index == prev_block.index + 1;

        let prev_hash_is_valid = block.prev_hash == prev_block.hash;

        let transactions_are_valid = self.validate_transactions(&block.transactions, blockchain);

        hash_is_valid && index_is_valid && prev_hash_is_valid && transactions_are_valid
    }  

    fn validate_transactions(&self, transactions: &[Transaction], blockchain: &Blockchain) -> bool {
        for transaction in transactions {
            if !self.is_transaction_valid(transaction, blockchain) {
                return false;
            }
        }
        true
    }

    fn is_transaction_valid(&self, transaction: &Transaction, blockchain: &Blockchain) -> bool {
    let sender_balance = blockchain.get_balance(&transaction.sender);

    let sender_balance_is_enough = (sender_balance > transaction.amount + transaction.fee);
    let fee_is_greater_that_zero = transaction.fee > 0;

    sender_balance_is_enough && sender_balance_is_enough ;
    true // Genesis block bootstrap transaction are infeasible if keypair do not already exists... 
    }

}
