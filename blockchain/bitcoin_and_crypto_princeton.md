# Bitcoin and Cryptocurrency Technologies - Princeton University

+ Bitcoin uses a timestamping system tot record ordering of transactions in a tamper-resistant way.
+ Bitcoin combines the idea of using computational puzzles to regulate the creation of new currency units with the idea of secure timestamping to record a ledger of transactions and prevent double spending.

# Chapter 1 - Introduction to Cryptography and Cryptocurrencies
+ Bitcoin uses cryptography to provide security and prevent cheating.
+ Cryptography provides a mechanism for securely encoding the rules of a cryptocurrency system in the system itself.

### Cryptographic Hash Functions
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

## Chapter 1.2 Hash Pointers and Data Structures.
+ A hash pointer is a data structure and a pointer to where some information is stored together with a cryptographic hash of the information.
+ A regular pointer gives you a way to retrieve the information a hash pointer gives you a way to retrieve the information as well as verify the information has not been modified.
+ A Blockchain is a linked list that is built with hash pointers instead of regular pointers.