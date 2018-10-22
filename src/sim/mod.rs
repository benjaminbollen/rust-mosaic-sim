//! Simulation universe

use ethereum_types::U256;
use types::blockchain::Blockchain;
use types::node::{Proposer, Validator};
use config;

mod actor;

pub struct World {
    timestep: u64,
    origin: Blockchain,
    cores: Vec<Blockchain>,
    proposers: Vec<Proposer>,
    validators: Vec<Validator>,
}

impl World {
    /// initialise a new world
    pub fn initialise(config: &config::Config) -> World {
        let mut account_counter = 0u64;
        let mut proposers : Vec<Proposer> = Vec::new();
        let mut validators : Vec<Validator> = Vec::new();
        // set initial balance to some sufficient value
        let balance: U256 = "10000000000000000000000000000".into();
        let origin = Blockchain::new(config.origin_block_time);
        let mut cores : Vec<Blockchain> = Vec::new();
        for _ in 0..config.cores-1 {
            cores.push(Blockchain::new(config.core_block_time));
            for _ in 0..config.core_proposers-1 {
                proposers.push(Proposer::new(account_counter,
                    balance.clone(), balance.clone()));
                account_counter += 1u64;
            }
            for _ in 0..config.core_validators-1 {
                validators.push(Validator::new(account_counter,
                    balance.clone(), balance.clone()));
                account_counter += 1u64;
            }
        }

        World {
            timestep: 0u64,
            origin: origin,
            cores: cores,
            proposers: proposers,
            validators: validators,
        }
    }

}