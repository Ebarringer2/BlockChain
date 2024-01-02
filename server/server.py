from flask import Flask, jsonify
from blockchain.merkle.merkle import MerkleTree
from blockchain.block.block import Block
from blockchain.proof.pow import Pow
from time import time

class Server:
    def __init__(self, mine_path : str, chain_path : str, hashes_path : str):
        self.app = Flask(__name__)
        self.blockchain = []
        self.num_mined : int = 0
        self.mine_path = mine_path
        self.chain_path = chain_path
        self.hashes_path = hashes_path
        self.setup_routes() 
        self.load_chain()
    def setup_routes(self):
        self.app.route('/mine', methods=['GET'])(self.mine_endpoint)
        self.app.route('/chain', methods=['GET'])(self.get_chain_endpoints)
    def calculate_merkle_root(self, transactions):
        merkle_tree = MerkleTree(transactions)
        return merkle_tree.get_root()
    def mine_endpoint(self):
        transactions = ["tx1", "tx2", "tx3"] 
        merkle_root = self.calculate_merkle_root(transactions)
        index = len(self.blockchain) + 1
        timestamp = time()
        prev_hash = self.blockchain[-1]['hash'] if self.blockchain else "0"
        block = Block(
            index,
            timestamp,
            transactions,
            prev_hash,
            proof=0
        )
        pow = Pow(block=block, difficulty=self.update_pow())
        start_time = time()
        valid_pow = pow.mine()
        #pow.visualize()
        block.proof = valid_pow 
        block_hash = pow.calculate_hash()
        end_time = time()
        elapsed = end_time - start_time
        self.blockchain.append({
            'index': block.index,
            'timestamp': block.timestamp,
            'transactions': block.transactions,
            'proof': block.proof,
            'prev_hash': block.prev_hash,
            'merkle_root': merkle_root,
            'hash': block_hash
        })
        self.num_mined += 1
        self.printgap()
        print(f'DIFFICULTY: {self.update_pow()}')
        with open(self.mine_path, 'a+') as f:
            f.write(f'{block_hash} MINED | start: {start_time} | elapsed: {elapsed}')
            f.write('\n')
        with open(self.chain_path, 'w') as f: f.write(str(self.blockchain))
        with open(self.hashes_path, 'a+') as f: f.write(f"BLOCK {block.index}: {block_hash}\n")
        return jsonify({'chain': self.blockchain, 'length': len(self.blockchain)}), 200
    def get_chain_endpoints(self): return jsonify({'chain': self.blockchain, 'length': len(self.blockchain)}), 200
    def run(self, host='0.0.0.0', port=5000): self.app.run(host=host, port=port)
    def update_pow(self): return len(self.blockchain)
    def printgap(self): print('\n----------------------------------------------------------------------\n')
    def load_chain(self):
        try:
            with open(self.chain_path, 'r') as f:
                chain = f.read()
                saved_blockchain = ''
                if chain:
                    saved_blockchain = eval(f.read())
                if isinstance(saved_blockchain, list):
                    self.printgap()
                    print('VALID BLOCKCHAIN FOUND')
                    self.printgap()
                    self.blockchain = saved_blockchain 
                else:
                    print('invalid format in chain file')
        except FileNotFoundError:
            print('\nChain file not found. Creating a new blockchain')
            self.printgap()