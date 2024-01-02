from node import P2PNode
from server.server import Server

class Trader:
    def __init__(self, node_1 : P2PNode, node_2 : P2PNode, server : Server, hashes_path : str):
        '''
        assumes that the hashes_path file and the server
        have had mining history such that there are block
        hashes to fetch
        '''
        self.node_1 = node_1
        self.node_2 = node_2
        self.server = server
        self.hashes_path = hashes_path
        self.blockchain = self.server.blockchain
        self.hashes = ''
        self.get_hashes()
    def get_hashes(self): 
        with open(self.hashes_path, 'r') as f: 
            self.hashes = f.splitlines()
    #def transaction(self, hash : str):
    