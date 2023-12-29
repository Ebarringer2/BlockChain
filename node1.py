from p2p.node import Node 

node_1 = Node('localhost', 3000)
node_1.connect('localhost', 5000)
node_1.send_data('hello')
node_1.start()