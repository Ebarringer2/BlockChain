use ring::digest;
use rand::Rng;

type HASH = Vec<u8>;

pub fn generate_rand_string() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    const STRING_LENGTH: usize = 10;
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let random_string: String = (0..STRING_LENGTH)
        .map(|_| {
            let index: usize = rng.gen_range(0..CHARSET.len());
            CHARSET[index] as char
        })
        .collect();
    random_string
}

pub fn calculate_hash(data: String) -> HASH {
    let mut context = digest::Context::new(&digest::SHA256);
    context.update(data.as_bytes());
    context.finish().as_ref().to_vec()
}

pub struct Block {
    pub index: i32,
    pub owner: String,
    pub timestamp: i32,
    pub transactions: Vec<String>,
    pub proof: i32,
    pub prev_hash: String
}

pub struct Pow {
    pub block: Block,
    pub difficulty: i32
}

pub struct MerkleTree {
    block: Block,
    transactions: Vec<String>,
    tree: Vec<HASH>
}

impl Block {
    pub fn new(index: i32, owner: String, timestamp: i32, transactions: Vec<String>, proof: i32, prev_hash: String) -> Self {
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

impl Pow {
    pub fn new(block: Block, difficulty: i32) -> Self {
        Pow {block, difficulty}
    }
    pub fn hash(&self, input: String) -> HASH {
        let mut context = digest::Context::new(&digest::SHA256);
        context.update(input.as_bytes());
        context.finish().as_ref().to_vec()
    }
    pub fn solve_proof(&self) -> i32 {
        let target_prefix = vec![0u8; (self.difficulty / 8) as usize];
        let mut nonce: i32 = 0;
        loop {
            nonce += 1;
            let input_data: String = format!(
                "{}{}{}{}{}",
                self.block.index,
                self.block.owner,
                self.block.timestamp,
                self.block.transactions.join(","),
                nonce
            );
            let hash: HASH = self.hash(input_data);
            if hash.starts_with(&target_prefix) {
                return nonce;
            }
        }
    }
}

impl MerkleTree {
    pub fn new(block: Block) -> Self {
        MerkleTree {
            block: block,
            transactions: block.transactions,
            tree: vec![]
        }
    }
    pub fn build_tree(&self) {
        if self.transactions.is_empty() {
            self.tree = vec![];
        }
        let mut tree: Vec<HASH> = self.transactions.iter().map(|tx: &String| self.hash_transaction(tx)).collect();
        while tree.len() > 1 {
            tree = self.combine_pairs(tree);
        self.tree = tree;
        }
    }
    pub fn hash_transaction(&self, transaction: String) -> HASH {
        calculate_hash(transaction)
    }
    pub fn combine_hashes(&self, left: HASH, right: HASH) -> HASH {
        left.extend(right);
        calculate_hash();
    }
}