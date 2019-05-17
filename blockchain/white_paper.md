# Bitcoin White Paper
+ Bitcoin can be seen fundamentally known as a solution to the double-spending problem using a peer-peer distrubuted timestamp server to generate computational proof of the chronological order of transactions.
+ An electronic paymenyt system based on cryptographic proof instead of trust
+ The system remains secure provided honest NODES collectively control superior CPU power than any cooperating group of attacker nodes.
+ Transactions are publicly announced, and a group of participants must agree on a single history of the order in which transactions where received.
+ A *Timestamp Server* takes on the task of timestamping and wildely publishing the the hash of a block of items .
+ The *proof of work* is scanning for a value that when hashed, the hash begins with a number of zero bits.
+ In the timestamp network, the proof-of-work is implemented by incrementing a _nonce_(number only used once) in the block until a value is found that when hashed, gives the block's hash the required number of zero bits.
+ Once the CPU effort has been expended to make it satisfy the proof-of-work, the block cannot be changed without redoing the work. As later blocks are chained after it, the work to change the block would include redoing all the blocks after it.
+ "Majority Decision making" is solved by Proof-of-work based on *one-CPU-one-vote*.
+ CPU and Electricity are expended to add new coins to the chain.
+ A block header with no transactions would be 640 bits (80 bytes)
+

### Network
The steps to run the network are as follows:
+ New transactions are broadcast to all nodes.
+ Each node collects new transactions into a block.
+ Each node works on finding a difficult proof-of-work for its block.
+ When a node finds a proof-of-work, it broadcasts the block to all nodes.
+ Nodes accept the block only if all transactions in it are valid and not already spent.
+ Nodes express their acceptance of the block by working on creating the next block in the chain, using the hash of the accepted block as the previous hash.