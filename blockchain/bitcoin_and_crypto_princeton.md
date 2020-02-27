# Bitcoin and Cryptocurrency Technologies - Princeton University

+ Bitcoin uses a timestamping system to record ordering of transactions in a tamper-resistant way.
+ Bitcoin combines the idea of using computational puzzles to regulate the creation of new currency units with the idea of secure timestamping to record a ledger of transactions and prevent double spending.

## Chapter 1 - Introduction to Cryptography and Cryptocurrencies
+ Bitcoin uses cryptography to provide security and prevent cheating.
+ Cryptography provides a mechanism for securely encoding the rules of a cryptocurrency system in the system itself.

### Section 1.1 - Cryptographic Hash Functions
+ A hash function is a mathematical function with the three following properties:
    - its input can be a string of any size (arbitrary length).
    - it produces a fixed size output. For the purpose of making the discussion in this chapter concrete, we will assume a 256‐bit output size. However, our discussion holds true for any output size as long as it is sufficiently large.
    - It is efficiently computable. Intuitively this means that for a given input string, you can figure out what the output of the hash function is in a reasonable amount of time. More technically, computing the hash of an n ‐bit string should have a running time that is O( n ).
+ A cryptographcally secure hash function must have the following properties: 1) Collosion-resitance 2) Hiding 3) puzzle-friendliness.
+ Puzzle friendliness is specific to cryptocurrencies not the general hash funtions requirements.
+ A hash function H is said to be collision resistant if it is infeasible to find two values, x and y, such that x ≠ y, yet H(x) = H(y).
+ Hashes are used to verify file integrity, say we generate a _message digest_ of a file we upload to a server and store it somewhere secure, we can compute the hash when we redownload our file and compare it to the one we stored to verify that it has not been modified. Hash functions provide the ability to perform such verification by being _collision-free_.
+ _Hiding_: A hash function H is hiding if: when a secret value r is chosen from a probability distribution that has high min‐entropy , then given H(r ‖ x) it is infeasible to find x .
+ _Min-Entropy_ is a measure of how predictable an outcome is, and high min-entropy captures the intuitive idea that the distribution of say, a random variable, is very spread out.
+ An Application of _Hiding_ can be something called _Commitment Scheme_.
+ _Puzzle Friendliness_ means that if someone wants to target the hash function to come out to some particular output value y, that if there’s part of the input that is chosen in a suitably randomized way, it’s very difficult to find another value that hits exactly that target. This can be explained further using a search puzzle:
    - Say we have a search puzzle that requires searching a very large space to find the solution to a mathematical problem and there are no shortcuts but to actually search the very large space.
    ```
       H(id || x) ∈ Y. [|| means concatenation]
        - H is the hash function
        - id is the puzzle ID -- chosen from a high min-entropy distribution
        - Y is the target set.
    ```
    - Our two inputs are `x` and `id`. The solution is finnding an input that the output falls within the set `Y`.
    - The value of set `Y` determines how hard the puzzle is.
    - The fact that the puzzle ID is choosen from a hight min-entropy distribution ensures that there are no shortcuts to solving the puzzle.
    - A search puzzle that is puzzle friendly implies that there are no strategies to solving the search puzzle.

+ Bitcoin uses _`SHA-256`_. SHA‐256 uses a compression function that takes 768‐bit input and produces 256‐bit outputs. The block size is 512 bits.
+SHA‐256 uses the Merkle‐Damgard transform to turn
a fixed‐length collision‐resistant compression function into a hash function that accepts arbitrary‐length inputs.
+ The _Merkle-Damgard transform_ is a generic method for converting a hash function that accept on fixed-length input to hash function that accept input of arbitrary length. When this happens the underlying fixed-length hash function is called _compression function_. If the comporession function is collision-resistant, the the overall hash function is collision-resistant as well.

### Section 1.2 - Hash Pointers and Data Structures.
+ A hash pointer is a data structure and a pointer to where some information is stored together with a cryptographic hash of the information at some fixed point in time.
+ A regular pointer gives you a way to retrieve the information, a hash pointer gives you a way to retrieve the information as well as verify the information has not been modified.
+ A Blockchain is a linked list that is built with hash pointers instead of regular pointers.
+ The Blockchain can be used to build a tamper-evident log, which means that suppose we have a log data structures that stores logs at given points in time by appending at the end of the log file, if a person tampers with a log at an earlier point in time we will detect it. Because the next log contains the hash of the log, it will not match up because our hash function is collision free we can therefore detect that the data has been modified.
+ The head of the block is called the genesis block.
+ In a _Merkle tree_, data blocks are grouped in pairs and the hash of each of these blocks is stored in a parent node. The parent nodes are in turn grouped in pairs and their hashes stored one level up the tree. This continues all the way up the tree until we reach the root node. It is a binary tree with hash pointers.
+ Merkle trees allows _Proof of membership_, To prove that a data block is included in the tree, one only needs to show the blocks in the path from that data block to the root. This takes *log(n)* to compute.
+ Just as _proof of membership_, we can also compute _proof of non-membership_ in logarithmic time and space. That is, we can prove that a particular block is not on the merkle tree:
+ The way we do that is simply by showing a path to the item that’s just before where the item in question would be and showing the path to the item that is just after where it would be. If these two items are consecutive in the tree, then this serves as a proof that the item in question is not included.

### Section 1.3 - Digital Signatures
+ Digital Signatures, along with hash functions are cryptographic primitives that serve as building blocks to a cryptocurrency.
+ (sk, pk) := generateKeys( keysize ) The generateKeys method takes a key size and generates a key pair. The secret key sk is kept privately and used to sign messages. pk is the public verification key that you give to everybody. Anyone with this key can verify your signature.
+ sig := sign( sk , message ) The sign method takes a message and a secret key, sk , as input and outputs a signature for message under sk.
+ isValid := verify( pk , message , sig ) The verify method takes a message, a signature, and a public key as input. It returns a boolean value, isValid , that will be true if sig is a valid signature for message under public key pk , and false otherwise.
+We say that the signature scheme is unforgeable if and only if, no matter what algorithm the adversary is using, his chance of successfully forging a message is extremely small — so small that we can assume it will never happen in practice.
+ If I sign a message with sk, my secret key, and someone later tries to validate that signature over that same message using my public key, pk, the signature must validate correctly. This property is a basic requirement for signatures to be useful at all.
+ Bitcoin uses `ECDSA` (Elliptic Curve Digital Signature Algorithm). More specifically `secp256k1` which is estimated to produce 128 bits of security.

#### Sizes of various quantities are:
+ Private key: 256 bits
+ Public key, uncompressed: 512 bits
+ Public key, compressed: 257 bits
+ Message to be signed: 256 bits
+ Signature: 512 bits

### Section 1.4 - PUblic Keys as Identities
+ The idea is to take a public key, one of those public verification keys from a digital signature scheme, and equate that to an identity of a person or an actor in a system.

### Section 1.5 - A simple Cryptocurrency
+ Eat your _cryptographic vegetables_. 😄

#### GoofyCoin