//! Validator

use ethereum_types::U256;

pub struct Proposer {
    account: u64,
    origin_balance: U256,
    core_balance: U256,
}

impl Proposer {
    pub fn new(account: u64, origin_balance: U256, core_balance: U256)
        -> Proposer {
        Proposer {
            account: account,
            origin_balance: origin_balance,
            core_balance: core_balance,
        }
    }
}

pub struct Validator {
    /// unique identifier of validator
    account: u64,
    origin_balance: U256,
    core_balance: U256,
}

impl Validator {
    pub fn new(account: u64, origin_balance: U256, core_balance: U256)
        -> Validator {
        Validator {
            account: account,
            origin_balance: origin_balance,
            core_balance: core_balance,
        }
    }
}