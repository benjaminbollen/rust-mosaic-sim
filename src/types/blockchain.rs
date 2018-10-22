// Copyright 2018 OpenST Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Blockchain has simulated contracts and blocks

use types::block::Block;

pub struct Blockchain {
    pub block_time: u64,
    pub blocks: Vec<Block>,
}

impl Blockchain {
    /// new blockchain
    pub fn new(block_time: u64) -> Blockchain {
        let blocks: Vec<Block> = Vec::new();

        Blockchain {
            block_time: block_time,
            blocks: blocks,
        }
    }
}