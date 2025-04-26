pub mod pow;

use crate::block::Block;
use crate::transaction::Transaction;
use crate::blockchain::Blockchain;

pub trait Consensus {
    fn mine_block(&self, transactions: &Vec<Transaction>, prev_hash: &String, blockchain: &Blockchain) -> Block;
    fn validate_block(&self, block: &Block, prev_block: &Block, blockchain: &Blockchain) -> bool;
    fn validate_transactions(&self, transactions: &[Transaction], blockchain: &Blockchain) -> bool;
    fn is_transaction_valid(&self, transaction: &Transaction, blockchain: &Blockchain) -> bool;
}
