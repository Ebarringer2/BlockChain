from server.server import Server 
import os 
from dotenv import load_dotenv

load_dotenv()
if __name__ == '__main__':
    server = Server(
        mine_path=os.getenv('MINE_PATH'),
        chain_path=os.getenv('CHAIN_PATH'),
        hashes_path=os.getenv('HASHES_PATH')
        )
    server.run()