class Block:
    def __init__(self, index, timestamp, transactions, prev_hash, proof, owner=None):
        self.index = index
        self.owner = owner
        self.timestamp = timestamp
        self.transactions = transactions
        self.proof = proof
        self.prev_hash = prev_hash
    def to_dict(self):
        return {
            'index': self.index,
            'timestamp': self.timestamp,
            'transactions': self.transactions,
            'proof': self.proof,
            'previous_hash': self.prev_hash,
            'owner': self.owner
        }