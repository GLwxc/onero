use std::collections::LinkedList;
use crate::{Block, Transaction, MemPool};
use crate::consensus::Consensus;
use crate::transaction::OptimizationFn;
use rand::Rng;
use sha2::{Digest, Sha256};
use crate::transaction::calculate_hash;

pub struct Blockchain {
    pub chain: LinkedList<Block>,
    pub consensus: Box<dyn Consensus>,
    pub difficulty: u32, // Fixed difficulty
}

impl Blockchain {
    pub fn new(consensus: Box<dyn Consensus>, difficulty: u32) -> Self {
        let mut chain = LinkedList::new();

        // Create initial "funding" transactions
        let initial_transactions = vec![
            Transaction {
                sender: "onerotoshi".to_string(), // Special "zero" address = genesis
                receiver: "onerotoshi".to_string(),
                amount: 1_000_000,
                fee: 0,
            },
            Transaction {
                sender: "onerotoshi".to_string(),
                receiver: "onerotoshi".to_string(),
                amount: 1_000_000,
                fee: 0,
            },
        ];

        let genesis_block = Block::new(
            0,
            1634227200, // Assuming a UNIX timestamp for simplicity
            initial_transactions,
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
    pub fn get_balance(&self, address: &str) -> u32 {
        let mut balance = 0;
        for block in &self.chain {
            for tx in &block.transactions {
                if tx.receiver == address {
                    balance += tx.amount;
                }
                if tx.sender == address {
                    balance -= tx.amount + tx.fee;
                }
            }
        }
        balance
    }
}
