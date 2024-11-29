use sha2::{Sha256, Digest};
use chrono::Utc;

pub struct ProofOfHistory {
    ledger: Vec<String>,
}

impl ProofOfHistory {
    pub fn new() -> Self {
        Self { ledger: vec![] }
    }

    pub fn generate_entry(&mut self, previous_hash: String) -> String {
        let timestamp = Utc::now().timestamp().to_string();
        let mut hasher = Sha256::new();
        hasher.update(previous_hash);
        hasher.update(timestamp.as_bytes());
        let new_hash = format!("{:x}", hasher.finalize());
        self.ledger.push(new_hash.clone());
        new_hash
    }

    pub fn verify_entry(&self, hash: &String) -> bool {
        self.ledger.contains(hash)
    }
}

