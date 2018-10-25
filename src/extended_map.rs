use hlt::position::Position;
use std::collections::HashSet;
use hlt::game_map::GameMap;
use hlt::map_cell::MapCell;

/* A game map with extended information and functionality. */
pub struct ExtendedMap {
    pub width: usize,
    pub height: usize,

    // Positions where ships will be in the next turn.
    collision_positions: HashSet<Position>,
}

impl ExtendedMap {
    pub fn from_game_map(game_map: &GameMap) -> ExtendedMap {
        ExtendedMap {
            width: game_map.width,
            height: game_map.height,
            collision_positions: HashSet::new()
        }
    }

    /// Call before a turn
    pub fn clear_reserved_cells(&mut self) {
        self.collision_positions.clear();
    }

    /// Returns true if the position is still free. That position
    /// will be marked as occupied.
    /// Should be used with
    /// ```
    /// if ex_map.try_reserve_cell(pos) {
    ///     ship.move(...)
    /// }
    /// ```
    pub fn try_reserve_cell(&mut self, position: &Position) -> bool {
        let option = self.collision_positions.get(position).cloned();

        if option.is_some() {
            return false;
        } else {
            self.collision_positions.insert(position.clone());
            return true;
        }
    }
}



