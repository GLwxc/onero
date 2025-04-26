use ring::digest;
use ring::rand::{SecureRandom, SystemRandom};
use ring::signature::{Ed25519KeyPair, KeyPair, UnparsedPublicKey, Signature};

use crate::{Transaction};

use std::fmt;

use hex;

//#[derive(Debug)] // Signature does not implement Debug trait
pub struct SignedTransaction {
  pub transaction: Transaction,
  pub signature: Signature,
}

impl fmt::Debug for SignedTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SignedTransaction")
            .field("transaction", &self.transaction)
            .field("signature", &hex::encode(self.signature.as_ref()))
            .finish()
    }
}

// Generate a digital signature for a transaction
pub fn sign(transaction: &Transaction, key_pair: &Ed25519KeyPair) -> Signature {
    let message = transaction_to_bytes(transaction);
    return key_pair.sign(&message);
}


// Helper function to convert a transaction to bytes for signing
fn transaction_to_bytes(transaction: &Transaction) -> Vec<u8> {
  bincode::serialize(transaction).expect("Failed to serialize Transaction")
}

impl SignedTransaction {
    pub fn new(transaction: &Transaction, key_pair: &Ed25519KeyPair) -> Self {
        let signature = sign(transaction, key_pair);
            return SignedTransaction {
                      transaction: transaction.clone(),
                      signature: signature, 
                   }
    }

// Verify the digital signature of a signed transaction
    pub fn is_transaction_valid(&self, public_key_bytes: &[u8]) -> bool {
      let public_key = UnparsedPublicKey::new(&ring::signature::ED25519, public_key_bytes);
      let message = transaction_to_bytes(&self.transaction);
      public_key.verify(&message, &self.signature.as_ref()).is_ok()
    }
}
