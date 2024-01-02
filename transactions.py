from p2p.transactions import Trader 
from p2p.node import P2PNode
import os 
from dotenv import load_dotenv
from server.server import Server

load_dotenv()
MINE_PATH = os.getenv('MINE_PATH')
HASHES_PATH = os.getenv('HASHES_PATH')
CHAIN_PATH = os.getenv('CHAIN_PATH')

n_1 = P2PNode(
    host='localhost',
    port=5000
    )

n_2 = P2PNode(
    host='localhost',
    port=6000
)

s = Server(
    mine_path=MINE_PATH,
    hashes_path=HASHES_PATH,
    chain_path=CHAIN_PATH
    )

t = Trader(
    
)