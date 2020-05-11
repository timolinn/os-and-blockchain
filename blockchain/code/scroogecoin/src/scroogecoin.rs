//! Scroogecoin is a simple cryptocurrency
//! Signed by Scrooge, each transaction is
//! represented as a block, these blocks are linked
//! together with hash pointers such that one can
//! Traverse through the block and see all the transactions
//! As they have happened
//! Each block has a hash pointer the points to the previous block
//! it also has a unique trasaction ID. Each hash pointer is verifiable
//! To have been signed by scrooge.
//! Here's how the coin is going to work.
//!     1. Create coins in a single transaction. This transaction is known as CreateCoins
//!     2. Multiple coins can be created in a single transaction with a transaction ID
//!     3. Each coin cotains a serial number, value and recepient (public key)
//!     4. A PayCoins transaction is valid if:
//!         a. coin is valid
//!         b. coin is not already consumed (double spend check)
//!         c. total value out = total value in
//!         d. signed by owners of all consumers

enum TransactionType {
    CreateCoins,
    PayCoins {
        consumed_coins: Vec<(i64, i64)>,// trans_id and serial number of the coin
        signatures: Vec<String>
    }
}

pub struct Transaction {
    trans_id: i64,
    pub trans_type: TransactionType,
    pub coins: Vec<Scroogecoin>
}

#[derive(Debug)]
pub struct Scroogecoin {
    trans_id: i64,
    num: i64,
    value: f32,
    recepient: String
}

impl Scroogecoin {
    pub fn coin_id(&self) -> (i64, i64) {
        (self.trans_id, self.num)
    }
}
