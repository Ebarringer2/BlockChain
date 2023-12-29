import hashlib
import json
import matplotlib.pyplot as plt

class Pow:
    def __init__(self, block, difficulty=4):
        self.block = block 
        self.difficulty = difficulty
    def mine(self):
        target = "0" * self.difficulty 
        proof = 0
        while True:
            self.block.proof = proof 
            #block_hash = self.calculate_hash()
            hashed_proof = self.hash_proof(proof)
            if hashed_proof.startswith(target):
                print(hashed_proof)
                break 
            else:
                proof += 1
                print(hashed_proof)
        return self.hash_proof(proof)
    def hash_proof(self, proof): return hashlib.sha256(str(proof).encode()).hexdigest()
    def calculate_hash(self):
        block_string = json.dumps(self.block.to_dict(), sort_keys=True)
        return hashlib.sha256(block_string.encode()).hexdigest()
    '''def visualize(self):
        proofs = []
        hashes = []
        target = "0" * self.difficulty 
        proof = 0 
        while True:
            self.block.proof = proof 
            block_hash = self.calculate_hash()
            proofs.append(proof)
            hashes.append(block_hash)
            if block_hash.startswith(target):
                break
            else:
                proof += 1
        plt.plot(proofs, hashes, marker='o')
        plt.xlabel('Proof')
        plt.ylabel('Hash')
        plt.grid(True)
        plt.show '''