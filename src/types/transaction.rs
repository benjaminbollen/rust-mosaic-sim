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

//! Transaction is a computational unit to transition the state of a blockchain
//! and can be typed for protocol related contracts

use std::collections;
use std::hash::{Hash, Hasher};

pub enum Action {
    /// Compute provides a baseline for useful computation, gas consumed
    Compute(u64),
    /// Vote for source to target validation
    Vote(VoteMessage),
}

pub struct Transaction {
    gas: u64,
    account: u64,
    core_id: u8,
}

pub struct VoteMessage {
    transition_hash: u64,
    source_hash: u64,
    target_hash: u64,
    source_height: u64,
    target_height: u64,
}