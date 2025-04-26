use ring::signature::{Ed25519KeyPair, KeyPair, Signature};
use crate::transaction::Transaction;
use crate::signature::SignedTransaction;
use ring::rand::{SystemRandom};

use std::fmt;

#[derive(Debug)]
pub struct Wallet {
    key_pair: Ed25519KeyPair, // Private key is kept secret
}

impl Wallet {
    pub fn new() -> Self {
      let rng = SystemRandom::new();
      let pkcs8_document = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
      let pkcs8_bytes = pkcs8_document.as_ref();
      let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes).unwrap();
      Wallet { key_pair }
    }

    pub fn new_from_keypair(key_pair: Ed25519KeyPair) -> Self {
        Wallet { key_pair }
    }

    pub fn public_key_bytes(&self) -> &[u8] {
        self.key_pair.public_key().as_ref()
    }

    pub fn sign_transaction(&self, transaction: &Transaction) -> SignedTransaction {
        SignedTransaction::new(transaction, &self.key_pair)
    }
}

