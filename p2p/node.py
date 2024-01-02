import socket
import threading
import hashlib
import os
from dotenv import load_dotenv

load_dotenv()
NODE_PATH = os.getenv('NODE_PATH')

class P2PNode:
    def __init__(self, host, port, name):
        self.host = host
        self.port = port
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.connections = []
        self.id = hashlib.sha256(name.encode()).hexdigest()
        with open(NODE_PATH, 'a+') as f: f.write(f"NAME: {name} | ID: {self.id}\n")
        self.start()
    def start(self):
        self.sock.bind((self.host, self.port))
        self.sock.listen(5)
        print(f"Node listening on {self.host}:{self.port}")
        accept_thread = threading.Thread(target=self.accept_connections)
        accept_thread.start()
    def accept_connections(self):
        while True:
            client, addr = self.sock.accept()
            self.connections.append(client)
            print(f"Connection established with {addr}")
            threading.Thread(target=self.handle_client, args=(client,)).start()
    def handle_client(self, client):
        while True:
            try:
                data = client.recv(1024)
                if not data:
                    break
                message = data.decode('utf-8')
                print(f"\nReceived message: {message}")
                self.broadcast(message, client)
            except Exception as e:
                print(f"Error handling client: {e}")
                break
    def broadcast(self, message, sender):
        for connection in self.connections:
            if connection != sender:
                try:
                    connection.send(message.encode('utf-8'))
                except Exception as e:
                    print(f"Error broadcasting message: {e}")
                    self.connections.remove(connection)
    def send_message(self, message, target_host, target_port):
        target = (target_host, target_port)
        try:
            with socket.create_connection(target) as temp_sock:
                temp_sock.send(message.encode('utf-8'))
        except Exception as e:
            print(f"Error sending message to {target}: {e}")
    def run(self):
        while True:
            target_host = input('\nTarget host: ')
            target_port = input('\nTarget port: ')
            message = input('\nMessage: ')
            if message == '/QUIT':
                break
            self.send_message(
                message=message,
                target_host=target_host,
                target_port=target_port
            )