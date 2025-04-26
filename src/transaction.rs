use std::collections::VecDeque;

use serde::Serialize;

pub type OptimizationFn = fn(&[&Transaction]) -> Vec<Transaction>;

#[derive(Debug, Clone)]// Define the Block struct
pub struct MemPool {
    pub proposed_transactions: VecDeque<Transaction>, // List of proposed transactions
}

#[derive(Debug, Clone, Serialize)] // Define the Transaction struct (assuming you have a separate transaction module)
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u32,
    pub fee: u32,
}

impl Transaction {
  pub fn as_raw(&self) -> Vec<u8> {
    bincode::serialize(self).expect("Failed to serialize Transaction")
  }
}

impl MemPool {
    // Constructor method to create a new mempool
    pub fn new() -> Self {
        MemPool {
          proposed_transactions: VecDeque::new(),
        }
    }
    pub fn extract_transactions(&self, max_transactions: usize, optimization_fn: OptimizationFn) -> Vec<Transaction> {
        let transaction_slice: Vec<&Transaction> = self.proposed_transactions.iter().collect();
        let transaction_slice = &transaction_slice[..];

        let optimized_transactions = optimization_fn(transaction_slice);

        optimized_transactions.into_iter().take(max_transactions).collect()
    }
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.proposed_transactions.push_back(transaction);
    }
    // This works, because we assume the transactions are treated FIFO
    pub fn remove_transactions(&mut self, transactions: &[Transaction]) {
        let transactions_size = transactions.len();
        for _ in 0..transactions_size {
            if self.proposed_transactions.pop_front().is_none() {
                // If the queue is empty, break out of the loop
                break;
            }
        }
    }
}
