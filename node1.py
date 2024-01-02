from p2p.node import P2PNode

node_1 = P2PNode(
    host='localhost', 
    port=5000, 
    name='Node 1'
    )
node_1.run()