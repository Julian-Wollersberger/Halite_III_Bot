pub mod simulator;
pub mod memory;
pub mod action;
pub mod logger;

mod turn_state;
mod state_difference;

//TODO Check if u16, u32 or usize is faster
pub type Halite = u32;

