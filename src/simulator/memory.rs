use simulator::action::Action;
use hlt::ShipId;
use std::collections::HashMap;

/// Persistent across turns.
/// The hlt_game replaces all its objects,
/// so none of them can be borrowed until the next turn.
pub struct Memory {
    ship_actions: HashMap<ShipId, Vec<Action>>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            ship_actions: HashMap::new(),
        }
    }
}