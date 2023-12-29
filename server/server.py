from flask import Flask, jsonify, request
from blockchain.merkle.merkle import MerkleTree
from blockchain.block.block import Block
from blockchain.proof.pow import Pow
from time import time
from random import randint

class Server:
    def __init__(self, save_file_path : str):
        self.app = Flask(__name__)
        self.blockchain = []
        self.num_mined : int = 0
        self.save_file_path = save_file_path
        self.setup_routes()
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
        with open(self.save_file_path, 'a+') as f:
            f.write(f'BLOCK MINED | start: {start_time} | end: {end_time} | elapsed: {elapsed}')
            f.write('\n')
        return jsonify({'chain': self.blockchain, 'length': len(self.blockchain)}), 200
    def get_chain_endpoints(self):  return jsonify({'chain': self.blockchain, 'length': len(self.blockchain)}), 200
    def run(self, host='0.0.0.0', port=5000): self.app.run(host=host, port=port)
    def update_pow(self): return self.num_mined
    def printgap(self): print('\n----------------------------------------------------------------------\n')