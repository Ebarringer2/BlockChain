mod blockchain;

use std::net::TcpListener;
//use blockchain::Block;
//use blockchain::MerkleTree;
//use blockchain::Pow;
use blockchain::Server;
use blockchain::ThreadPool;

fn main() {
    let adress: String = "localhost:3000".to_string();
    // for dev purpose
    let path: String = "NONE".to_string();
    let server: Server = Server::new(adress.clone(), path.clone(), path.clone(), path.clone());
    let pool: ThreadPool = ThreadPool::new(4);
    let listener: TcpListener = TcpListener::bind(server.adress.clone()).unwrap();
    for stream in listener.incoming().take(2) {
        let stream: std::net::TcpStream = stream.unwrap();
        pool.execute(|| {
            server.handle_connection(stream)
        });
    }
}