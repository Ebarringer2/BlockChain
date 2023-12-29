import hashlib 

class MerkleTree:
    def __init__(self, transactions):
        self.transactions = transactions
        self.tree = self.buildTree()
    def buildTree(self):
        if not self.transactions:
            return []
        tree = [self.hash_transaction(tx) for tx in self.transactions]
        while len(tree) > 1:
            tree = self.combine_pairs(tree)
        return tree 
    def combine_pairs(self, pairs):
        result = []
        for i in range(0, len(pairs), 2):
            left = pairs[i]
            right = pairs[i + 1] if i + 1 < len(pairs) else left 
            combined = self.combine_hashes(left, right)
            result.append(combined)
        return result
    def combine_hashes(self, left, right):
        combined = left + right
        return hashlib.sha256(combined.encode()).hexdigest()
    def hash_transaction(self, transaction): return hashlib.sha256(str(transaction).encode()).hexdigest()
    def get_root(self): return None if not self.tree else self.tree[0]