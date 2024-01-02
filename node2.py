from p2p.node import P2PNode

node_2 = P2PNode(
    host='localhost', 
    port=6000,
    name='Node 2'
    )
node_2.run()