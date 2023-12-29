from p2p.node import P2PNode 
import threading

node = P2PNode('localhost', 5000)
threading.Thread(target=node.start()).start()
while True:
    message = input('Enter message to send: ')
    node.send_message(message)