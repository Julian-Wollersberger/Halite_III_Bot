use hlt::position::Position;
use std::collections::HashSet;
use hlt::game_map::GameMap;
use std::cell::RefCell;
use simulator::logger::log;

/// A game map with extended information and functionality.
/// Collision Avoidance.
/// From commit 962ab1a62fc9132a67e49d7456b92ded34178b60 extended_map.rs
pub struct CollisionAvoidance<'game> {
    pub game_map: &'game GameMap,
    /// Positions where ships will be in the next turn.
    collision_positions: RefCell<HashSet<Position>>,
}

impl<'game> CollisionAvoidance<'game> {
    pub fn new(game_map: &'game GameMap) -> CollisionAvoidance {
        CollisionAvoidance {
            game_map,
            collision_positions: RefCell::new(HashSet::new()),
        }
    }
    
    /// Collision Avoidance.
    /// Returns true if the position is still free. That position
    /// will be marked as occupied.
    /// Should be used with
    /// ```
    /// if ex_map.try_reserve_cell(pos) {
    ///     ship.move(...)
    /// }
    /// ```
    pub fn can_move_safely_then_reserve(&self, position: &Position) -> bool {
        // Don't movement to an occupied cell
        if self.game_map.at_position(position).ship.is_some() {
            log(&format!("Cell occupied: pos {:?}", position));
            return false;
        }
        
        // Try reserve position
        if self.collision_positions.borrow().get(position).cloned().is_some() {
            log(&format!("Already reserved: pos {:?}", position));
            return false;
        } else { // not occupied
            self.collision_positions.borrow_mut().insert(position.clone());
            log(&format!("Can move safe: pos {:?}", position));
            return true;
        }
    }
}



