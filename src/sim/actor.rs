//! Actor evaluates its possible actions on each timestep 
//! and can take no, or multiple actions in that timestep.

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

pub trait Actor {
    /// Actor evaluates 
    fn evaluate(&self);
}

