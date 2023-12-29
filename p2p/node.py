import socket 
import threading 

class Node:
    def __init__(self, host, port):
        self.host = host 
        self.port = port 
        self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.connections = []
    def connect(self, peer_host, peer_port):
        try:
            connection = self.socket.connect((peer_host, peer_port))
            self.connections.append(connection)
            print(f'Connected to {peer_host}:{peer_port}')
        except socket.error as e:
            print(f"Failed to connect to {peer_host}:{peer_port}. Error: {e}")
    def listen(self):
        self.socket.bind((self.host, self.port))
        self.socket.listen(10)
        print(f"Listening for connections on {self.host}:{self.port}")
        while True:
            connection, adress = self.socket.accept()
            self.connections.append(connection)
            print(self.connections)
            print(f"Accepted connection from {adress}") 
    def send_data(self, data):
        for connection in self.connections:
            print(f'Sending data from connection {connection}')
            try:
                connection.sendall(data.encode())
            except socket.error as e:
                print(f'Failed to send data. Error: {e}')
    def start(self):
        listen_thread = threading.Thread(target=self.listen)
        listen_thread.start()