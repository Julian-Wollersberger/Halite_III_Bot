use hlt::position::Position;
use std::collections::HashSet;
use hlt::game_map::GameMap;

/// A game map with extended information and functionality.
/// Collision Avoidance.
pub struct ExtendedMap<'game> {
    pub game_map: &'game GameMap,
    /// Positions where ships will be in the next turn.
    collision_positions: HashSet<Position>,
}

impl<'game> ExtendedMap<'game> {
    pub fn new(game_map: &'game GameMap) -> ExtendedMap {
        ExtendedMap {
            game_map,
            collision_positions: HashSet::new()
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
    pub fn can_move_safely_then_reserve(&mut self, position: &Position) -> bool {
        // Don't movement to an occupied cell
        if self.game_map.at_position(position).ship.is_some() {
            return false;
        }

        // Try reserve position
        if self.collision_positions.get(position).cloned().is_some() {
            return false;
        } else { // not occupied
            self.collision_positions.insert(position.clone());
            return true;
        }
    }
}



