// mods
use ring::digest;
use rand::Rng;
use std::error;
use std::fmt;
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    sync::{mpsc, Arc, Mutex},
    thread,
    fs,
    time::Duration
};

// types
type HASH = Vec<u8>;
type TREE = Vec<String>;
type TreeErrHandle = Result<TREE, SizeError>;
type BLOCKCHAIN = Vec<Block>;
type Job = Box<dyn FnOnce() + Send + 'static>;
type StringArr<const N: usize> = [char; N];

// consts
const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const STRING_LENGTH: usize = 10;


pub fn generate_rand_string() -> String {
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

pub fn string_to_arr<const N: usize>(string: &str) ->  [char; N] {
    let mut data: [char; N] = ['\0'; N];
    let chars: Vec<char> = string.chars().collect();
    data[..chars.len()].copy_from_slice(&chars);
    return data
}

pub fn arr_to_string(arr: &[char]) -> String {
    arr.iter().collect()
}

#[derive(Debug, Clone)]
struct SizeError(&'static str);

#[derive(Debug, Clone)]
struct AttributeError(&'static str);

pub struct HashtoString {
    pub hash: HASH,
    pub string: String,
}

pub struct StringtoHash {
    pub string: String,
    pub hash: HASH
}

#[derive(Debug)]
/// IMPORTANT
/// 
/// length of owner arr is capped at 25
/// 
/// make sure the attribute here can be stored in a
/// 
/// [char; 25] suitably
pub struct Block {
    pub index: i32, 
    pub owner: StringArr<25>,
    pub timestamp: i32,
    pub transactions: Vec<String>,
    pub proof: i32,
    pub prev_hash: HASH
}

//impl Copy for Block {}

pub struct Pow {
    pub block: Block,
    pub difficulty: i32
}

pub struct MerkleTree {
    pub block: Block,
    pub transactions: Vec<String>,
    pub tree: Result<TREE, SizeError> // for error handling
}

#[derive(Debug)]
pub struct Server {
    pub blockchain: BLOCKCHAIN,
    //pub listener: TcpListener,
    pub adress: String,
    pub num_mined: i32,
    pub mine_path: String,
    pub chain_path: String,
    pub hashes_path: String,
    //pub pool: ThreadPool,
    pub receiving: bool,
    pub mineable: bool
}

//impl Copy for Server {}

//    impl Clone for Server {
//        fn clone(&self) -> Self {
//            *self
//        }
//    }

pub struct ThreadPool {
    pub workers: Vec<Worker>,
    pub sender: Option<mpsc::Sender<Job>>,
}

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
    pub receiver: Arc<Mutex<mpsc::Receiver<Job>>>
}

impl fmt::Display for SizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SIZING ERROR: {}", self.0)
    }
}

impl error::Error for SizeError {}

impl fmt::Display for AttributeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ATTRIBUTE ERROR: {}", self.0)
    }
}

impl error::Error for AttributeError {}

impl HashtoString {
    pub fn new(hash: HASH) -> Self {
        let mut string: String = String::new();
        match String::from_utf8(hash.clone()) {
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
        return self.string.clone();
    }
}

impl Block {
    pub fn new(index: i32, owner: [char; 25], timestamp: i32, transactions: Vec<String>, proof: i32, prev_hash: HASH) -> Self {
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
        let block_transactions: Vec<String> = block.transactions.clone();
        MerkleTree {
            block: block,
            transactions: block_transactions,
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
    pub fn build_tree(&mut self) {
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
    pub fn combine_pairs(&self, pairs: TREE) -> TreeErrHandle {
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
    /// method for calculating the Merkle Root of the Merkle
    /// Tree object 
    /// first unwraps the tree attribute, allowing handling for the 
    /// case at which the tree contains a SizeError
    /// and handles the case at which the tree is valid
    pub fn calculate_root(&self) -> Result<String, SizeError> {
        match self.get_tree() {
            Ok(tree) => {
                if tree.is_empty() {
                    return Err(SizeError("Merkle tree cannot be empty when trying to calculate Merkle Root"))
                } else {
                    return Ok(tree[0].clone())
                }
            } 
            Err(_err) => {
                return Err(SizeError("Merkle tree cannot be empty when trying to calculate Merkle Root"))
            }
        }
    }
}

impl Server {
    /// #### creates a new Server object
    /// 
    /// #### The adress argument must be of structure Host:Port
    /// ## Example
    /// #### let adress = "127.0.0.1:7878"
    /// ## Params
    /// #### the path params specify the location of .txt files to read
    /// #### information from regarding the blockchain history
    /// #### IMPORTANT
    /// #### receiving and minable status of the server are preset to be false,
    /// #### so before the server is accessable, one must change these attributes
    pub fn new(adress: String, mine_path: String, chain_path: String, hashes_path: String) -> Self {
        //let listener: TcpListener = TcpListener::bind(adress.clone()).unwrap();
        let num_mined: i32 = 0;
        let blockchain: BLOCKCHAIN = Vec::new();
        //let pool: ThreadPool = ThreadPool::new(4);
        Server {
            blockchain,
            //listener,
            adress,
            num_mined,
            mine_path,
            chain_path,
            hashes_path,
            receiving: false,
            mineable: false
        }
    }
    pub fn run(server: Server, listener: TcpListener, pool: ThreadPool) -> Result<(), AttributeError> {
        if !server.receiving {
            return Err(AttributeError("Server object must have attribute 'receiving' set to true"));
        }
        //let server_arc = Arc::new(server.clone());
        let server_arc: Arc<Server> = Arc::new(server);
        while server_arc.receiving { 
                for stream in listener.incoming().take(2) {
                    let stream: TcpStream = stream.unwrap();
                    //let server_clone = Arc::clone(&server_arc);
                    println!("RECEIVED CONNECTION: {:#?}", stream);
                    pool.execute(move || {
                        server.handle_connection(stream);
                    })
                }    
        } 
        return Ok(());
    }  
    pub fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer: [u8; 1024] = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let get = b"GET / HTTP/1.1\r\n";
        let sleep = b"GET /sleep HTTP/1.1\r\n";
        let (status_line, filename) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "hello.html")
        } else if buffer.starts_with(sleep) {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };
        let contents = fs::read_to_string(filename).unwrap();
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }  
}

impl ThreadPool {
    /// create a new ThreadPool
    /// 
    /// size is the number of threads in the pool
    /// 
    /// # Panics
    /// 
    /// new function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver: Arc<Mutex<mpsc::Receiver<Box<dyn FnOnce() + Send>>>> = Arc::new(Mutex::new(receiver));
        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender: Some(sender)
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job: Job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let cloned_receiver = Arc::clone(&receiver);
        let thread: thread::JoinHandle<_> = thread::spawn(move || loop {
            let message: Result<Box<dyn FnOnce() + Send>, mpsc::RecvError> = cloned_receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job| EXECUTING");
                    job();
                }
                Err(_err) => {
                    println!("Worker {id} disconnected| SHUTTING DOWN");
                    break;
                }
            }
        });
        Worker {
            id, 
            thread: Some(thread),
            receiver
        }
    }
}