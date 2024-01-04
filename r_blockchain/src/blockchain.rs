use ring::aead::Tag;
use ring::digest;
use rand::Rng;
use std::error;
use std::fmt;

type HASH = Vec<u8>;
type TREE = Vec<String>;
type TREE_ERR_HANDLE = Result<TREE, SizeError>;

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

#[derive(Debug, Clone)]
struct SizeError(&'static str);

pub struct HashtoString {
    pub hash: HASH,
    pub string: String,
}

pub struct StringtoHash {
    pub string: String,
    pub hash: HASH
}

pub struct Block {
    pub index: i32,
    pub owner: String,
    pub timestamp: i32,
    pub transactions: Vec<String>,
    pub proof: i32,
    pub prev_hash: HASH
}

pub struct Pow {
    pub block: Block,
    pub difficulty: i32
}

pub struct MerkleTree {
    block: Block,
    transactions: Vec<String>,
    tree: Result<TREE, SizeError> // for error handling
}

impl fmt::Display for SizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SIZING ERROR: {}", self.0)
    }
}

impl error::Error for SizeError {}

impl HashtoString {
    pub fn new(hash: HASH) -> Self {
        let mut string: String;
        match String::from_utf8(hash) {
            Ok(s) => {
                string = s;
            }
            Err(e) => {
                println!("Error converting to String: {}", e);
            }
        }
        HashtoString {
            hash,
            string
        }
    }
    pub fn get_string(&self) -> String {
        return self.string;
    }
}


impl Block {
    pub fn new(index: i32, owner: String, timestamp: i32, transactions: Vec<String>, proof: i32, prev_hash: HASH) -> Self {
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
            // initialize tree as an empty Vec<String> for result enum
            tree: Ok(Vec::new())
        }
    }
    pub fn get_tree(&self) -> Result<&TREE, &SizeError> {
        // returns reference to TREE inside enum if no error -> the Ok
        // returns reference to the Err if Err -> Err
        match &self.tree {
            Ok(tree) => Ok(tree),
            Err(err) => Err(err)
        }
    }
    pub fn hash_to_string(&self, hash: HASH) -> String {
        let converter: HashtoString = HashtoString::new(hash);
        return converter.get_string()
    }
    pub fn build_tree(&self) {
        if self.transactions.is_empty() {
            // result enum here again
            self.tree = Ok(Vec::new());
        } else {
            let mut tree: TREE = self.transactions
                .iter()
                .map(|tx: &String| self.hash_transaction(tx.to_string()))
                .map(|hash: HASH| self.hash_to_string(hash))
                .collect();
            while tree.len() > 1 {
                match self.combine_pairs(tree) {
                    // size error avoided
                    Ok(new_tree) => tree = new_tree,
                    // catch size error
                    Err(err) => {
                        self.tree = Err(err);
                        return;
                    }
                }
            }
            self.tree = Ok(tree);
        }
    }
    pub fn hash_transaction(&self, transaction: String) -> HASH {
        calculate_hash(transaction)
    }
    pub fn combine_hashes(&self, left: HASH, right: HASH) -> HASH {
        let left_converter: HashtoString = HashtoString::new(left);
        let left_string: String = left_converter.get_string();
        let right_converter: HashtoString = HashtoString::new(right);
        let right_string: String = right_converter.get_string();
        let combined: String = left_string + &right_string;
        return calculate_hash(combined);
    }
    pub fn combine_pairs(&self, pairs: TREE) -> TREE_ERR_HANDLE {
        let mut result: TREE = Vec::new();
        if pairs.is_empty() {
            return Err(SizeError("length of pairs must be greater than 0"));
        }
        for i in (0..pairs.len()).step_by(2) {
            let left: String = pairs[i].clone();
            let right: String = if i + 1 < pairs.len() {
                pairs[i + 1].clone()
            } else { 
                left.clone()
            };
            let combined_str: String = left + &right;
            let combined_hash: HASH = calculate_hash(combined_str);
            let done: String = self.hash_to_string(combined_hash);
            result.push(done)
        }
        Ok(result)
    }
    pub fn calculate_root(&self) -> Result<String, SizeError> {
        match self.get_tree() {
            Ok(tree) => {
                if tree.is_empty() {
                    return Err(SizeError("Merkle tree cannot be empty when trying to calculate Merkle Root"))
                } else {
                    return Ok(tree[0].clone())
                }
            } 
            Err(err) => {
                return Err(SizeError("Merkle tree cannot be empty when trying to calculate Merkle Root"))
            }
        }
    }
}