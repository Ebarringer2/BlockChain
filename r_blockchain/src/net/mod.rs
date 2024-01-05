use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}
};

type BLOCKCHAIN = Vec<

pub struct Server {
    pub blockchain: BLOCKCHAIN,
    pub listener: TcpListener,
    pub adress: String,
    pub num_mined: i32,
    pub mine_path: String,
    pub chain_path: String,
    pub hashes_path: String,
    pub receiving: bool,
    pub mineable: bool
}