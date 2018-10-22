//! config for simulation run

pub struct Config {
    /// number of block proposers for a auxiliary chain
    pub core_proposers: u32,
    /// number of validators for an auxiliary chain
    pub core_validators: u32,
    /// number of auxiliary chains
    pub cores: u32,
    /// number of simulation steps per origin block
    pub origin_block_time: u64,
    /// number of simulation steps per auxiliary block
    pub core_block_time: u64,
}

impl Config {
    pub fn new(
        core_proposers: u32,
        core_validators: u32,
        cores: u32,
        origin_block_time: u64,
        core_block_time: u64) -> Config {
        
        Config {
            core_proposers: core_proposers,
            core_validators: core_validators,
            cores: cores,
            origin_block_time: origin_block_time,
            core_block_time: core_block_time,
        }
    }

    pub fn default() -> Config {
        // set granularity to the number of evaluation steps
        // per shortest block time.
        let granularity = 3u64;
        let origin_block_time = 5u64 * granularity;
        let core_block_time = granularity;

        Config {
            core_proposers: 5u32,
            core_validators: 20u32,
            cores: 1u32,
            origin_block_time: origin_block_time,
            core_block_time: core_block_time,
        }
    }
}