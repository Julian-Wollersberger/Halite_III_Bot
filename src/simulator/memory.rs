use simulator::action::Action;
use hlt::ShipId;
use std::collections::HashMap;
use hlt::direction::Direction;
use std::cell::RefCell;

/// Persistent across turns.
/// The hlt_game replaces all its objects,
/// so none of them can be borrowed until the next turn.
pub struct Memory {
    ship_moves: RefCell<HashMap<ShipId, Vec<Direction>>>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            ship_moves: RefCell::new(HashMap::new()),
        }
    }

    pub fn store_moves(&self, id: ShipId, dir: Vec<Direction>) {
        self.ship_moves.borrow_mut().insert(id, dir);
    }
    /// Remove from memory.
    pub fn moves_of_ship(&self, id: &ShipId) -> Vec<Direction> {
        match self.ship_moves.borrow_mut().remove(id) {
            Some(dir) => dir,
            None => Vec::new(),
        }
    }
}