//! Block

use types::transaction::Transaction;

pub struct Block {
    bytes: [u8; 20] // random
    // header: Header,
    // transactions: Vec<Transaction>,
}

// pub struct Header {
//     bytes: [u8; 20] // random
// }

impl Block {
    pub fn new() -> Block {
        Block {
            bytes: [0u8; 20]
        }
    }
}