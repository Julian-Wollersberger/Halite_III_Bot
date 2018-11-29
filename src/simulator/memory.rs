use simulator::state_difference::StateDifference;
use std::collections::HashMap;
use hlt::ShipId;
use hlt::direction::Direction;
use std::cell::RefCell;

/// Persistent across turns.
/// The hlt_game replaces all its objects,
/// so none of them can be borrowed until the next turn.
pub struct Memory {
    ship_path: RefCell<HashMap<ShipId, Vec<Direction>>>,
    //<turn_number, diff>
    diffs: RefCell<HashMap<u32, StateDifference>>,
}

impl Memory {
    pub fn new()-> Memory {
        Memory{
            ship_path: RefCell::new(HashMap::new()),
            diffs: RefCell::new(HashMap::new()),
        }
    }
    
    pub fn load_diff(&self, turn_number: u32) -> StateDifference {
        self.diffs.borrow_mut().remove(&turn_number)
            .or_else(|| Some(StateDifference::new()))
            .unwrap()
    }
    pub fn safe_diff(&self, turn_number: u32, diff: StateDifference) {
        self.diffs.borrow_mut().insert(turn_number, diff);
    }
    
    pub fn store_path(&self, id: ShipId, dir: Vec<Direction>) {
        self.ship_path.borrow_mut().insert(id, dir);
    }
    pub fn ship_path(&self, id: &ShipId) -> Vec<Direction> {
        match self.ship_path.borrow_mut().remove(id) {
            Some(dir) => dir,
            None => Vec::new(),
        }
    }
}