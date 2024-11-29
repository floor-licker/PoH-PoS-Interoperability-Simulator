pub struct ProofOfStake {
    finalized_blocks: Vec<String>,
}

impl ProofOfStake {
    pub fn new() -> Self {
        Self { finalized_blocks: vec![] }
    }

    pub fn finalize_block(&mut self, block_hash: String) {
        self.finalized_blocks.push(block_hash);
    }

    pub fn verify_finality(&self, block_hash: &String) -> bool {
        self.finalized_blocks.contains(block_hash)
    }
}

