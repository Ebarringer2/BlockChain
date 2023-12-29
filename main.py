from server.server import Server 
import os 
from dotenv import load_dotenv

load_dotenv()
if __name__ == '__main__':
    server = Server(os.getenv('SAVE_FILE_PATH'))
    server.run()