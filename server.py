from p2p.node import Node 

node = Node('localhost', 5000)
#node.connect('localhost', 3000)
node.start()