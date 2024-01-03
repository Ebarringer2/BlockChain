pub struct Block {
    pub index: i32,
    pub owner: String,
    pub timestamp: i32,
    pub transactions: Vec<String>,
    pub proof: String,
    pub prev_hash: String
}

impl Block {
    pub fn new(index: i32, owner: String, timestamp: i32, transactions: Vec<String>, proof: String, prev_hash: String) -> Self {
        Block {
            index,
            owner,
            timestamp,
            transactions,
            proof,
            prev_hash
        }
    }
}