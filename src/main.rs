mod block;
mod blockchain;
mod transaction;
mod signature;
mod wallet;

use block::{Block};
use blockchain::{Blockchain, ProofOfWork};
use transaction::{Transaction, MemPool, OptimizationFn};
use signature::{SignedTransaction};
use wallet::{Wallet};

use ring::digest;
use ring::rand::{SecureRandom, SystemRandom};
use ring::signature::{Ed25519KeyPair, KeyPair, UnparsedPublicKey, Signature};

use std::collections::VecDeque;

use serde::Serialize;

use hex::encode;

fn main() {
    // Create a new blockchain
    let pow = Box::new(ProofOfWork);
    let difficulty = 3;
    let mut mempool = MemPool::new();
    let mut onero_blockchain_pow = Blockchain::new(pow, difficulty);

    // Example optimization function
    let optimization_fn: OptimizationFn = |transactions| {
        let mut sorted_transactions: Vec<&Transaction> = transactions.to_vec();
        // Sort by fee in descending order
        sorted_transactions.sort_by(|a, b| b.fee.cmp(&a.fee));

        // Clone and collect into Vec<Transaction>
        sorted_transactions.into_iter().map(|t| (*t).clone()).collect()
    };

    for _ in 0..10 {
        mempool.add_transaction(
          Transaction { 
            sender: "1".to_string(),
            receiver: "1".to_string(),
            amount: 10,
            fee: 0,
          } 
        );
    }

    let bob_wallet = Wallet::new();
    let alice_wallet = Wallet::new();

    println!("bob wallet : {:?}", bob_wallet);
    println!("alice wallet : {:?}", alice_wallet);

    let bob_transaction = Transaction {
            sender: hex::encode(bob_wallet.public_key_bytes()),
            receiver: "1".to_string(),
            amount: 10,
            fee: 0,
    };

    println!("bob_transaction : {:?}", bob_transaction);

    mempool.add_transaction(bob_transaction.clone());

    let bob_signed_transaction = bob_wallet.sign_transaction(&bob_transaction);

    println!("bob_signed_transaction : {:?}", bob_signed_transaction);

    // Assuming you have access to the mempool variable at this point in your code
    println!("Mempool Contents Before Block Addition:");
    for transaction in mempool.proposed_transactions.iter() {
        println!("  Transaction: Sender: {}, Receiver: {}, Amount: {}, Fee: {}",
                 transaction.sender, transaction.receiver, transaction.amount, transaction.fee);
    }

    onero_blockchain_pow.add_block(&mut mempool, optimization_fn);
    onero_blockchain_pow.add_block(&mut mempool, optimization_fn);
    onero_blockchain_pow.add_block(&mut mempool, optimization_fn);
    onero_blockchain_pow.add_block(&mut mempool, optimization_fn);
    onero_blockchain_pow.add_block(&mut mempool, optimization_fn);
    onero_blockchain_pow.add_block(&mut mempool, optimization_fn);
    onero_blockchain_pow.add_block(&mut mempool, optimization_fn);


    let rng = SystemRandom::new();
    let pkcs8_document = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    let pkcs8_bytes = pkcs8_document.as_ref();
    let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes).unwrap();

    let mut signed_transactions = VecDeque::new();
    for transaction in mempool.proposed_transactions.iter() {
       let signed_transaction = SignedTransaction::new(&transaction, &key_pair);
       println!("signed transaction : {:?}", signed_transaction);
       let public_key_bytes = key_pair.public_key().as_ref();
       if signed_transaction.is_transaction_valid(&public_key_bytes) {
         signed_transactions.push_back(signed_transaction); 
      }
    }

    println!("Mempool Contents After Block Addition:");
    for transaction in mempool.proposed_transactions.iter() {
        println!("  Transaction: Sender: {}, Receiver: {}, Amount: {}, Fee: {}",
                 transaction.sender, transaction.receiver, transaction.amount, transaction.fee);
    }

    // Print the blockchain
    for block in onero_blockchain_pow.chain.iter() {
      println!("Block {}: ", block.index);
      println!("  Transactions: {:?}", block.transactions);
      println!("  Previous Hash: {:?}", block.prev_hash);
      println!("  Hash: {:?}", block.hash);
      println!("  Nonce: {:?}", block.nonce);
    }
}
