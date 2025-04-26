use std::collections::LinkedList;
use crate::{Block, Transaction, MemPool};
use crate::transaction::OptimizationFn;
use rand::Rng;
use sha2::{Digest, Sha256};

pub struct Blockchain {
    pub chain: LinkedList<Block>,
    pub consensus: Box<dyn Consensus>,
    pub difficulty: u32, // Fixed difficulty
}

impl Blockchain {
    pub fn new(consensus: Box<dyn Consensus>, difficulty: u32) -> Self {
        let mut chain = LinkedList::new();
        let genesis_block = Block::new(
            0,
            1634227200, // Assuming a UNIX timestamp for simplicity
            vec![],
            "0".to_string(), // Assuming this is the first block
            "".to_string(),  // Will be calculated later based on the genesis_block itself
            0,               // First nonce
        );
        let _genesis_block_hash = calculate_hash(&genesis_block.nonce, 
          &genesis_block.transactions, 
          &genesis_block.prev_hash);
        let mut genesis_block = genesis_block;
        genesis_block.hash = _genesis_block_hash;
        chain.push_back(genesis_block);
        Blockchain { chain, consensus, difficulty}
    }

    pub fn add_block(&mut self, mempool: &mut MemPool, optimization_fn: OptimizationFn) {
        // You might want to add some validation or consensus mechanism here before adding the block
        // extract transactions from the mempool
        let max_n_transactions = 3; // Example value
        let transactions = mempool.extract_transactions(max_n_transactions, optimization_fn);

        let prev_block = self.chain.back().unwrap();
        let new_block = self.consensus.mine_block(&transactions, &prev_block.hash, &self);

        if self.consensus.validate_block(&new_block, prev_block, &self) {
            self.chain.push_back(new_block);
            // then remove the transactions from mempool
            mempool.remove_transactions(&transactions);
        } else {
            // Handle invalid block
            panic!("You mined an invalid block. Abort.")
        }
    }
}

pub trait Consensus {
    fn mine_block(&self, transactions: &Vec<Transaction>, prev_hash: &String, blockchain: &Blockchain) -> Block;
    fn validate_block(&self, block: &Block, prev_block: &Block, blockchain: &Blockchain) -> bool;
    fn validate_transactions(&self, transactions: &[Transaction], blockchain: &Blockchain) -> bool;
    fn is_transaction_valid(&self, transaction: &Transaction, blockchain: &Blockchain) -> bool;
}

pub fn calculate_hash(nonce: &u32, transactions: &Vec<Transaction>, prev_hash: &String) -> String {
  let mut hasher = Sha256::new();
  hasher.update(format!(
      "{:?}{}{}",
     transactions, prev_hash, nonce
  ));

  let result = hasher.finalize();
  format!("{:x}", result)
}

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

//pub struct ProofOfStake;
//
//impl Consensus for ProofOfStake {
//    fn mine_block(&self, transactions: &Vec<Transaction>, prev_hash: &String, blockchain: &Blockchain) -> Block {
//        // Implement PoS logic here
//    }
//
//    fn validate_block(&self, block: &Block, prev_block: &Block, blockchain: &Blockchain) -> bool {
//        // Implement PoS validation logic here
//    }
//}
