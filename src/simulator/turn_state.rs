use std::collections::HashMap;
use std::prelude::v1::Vec;

use hlt::game::Game;
use hlt::position::Position;
use hlt::ship::Ship;
use hlt::ShipId;
use simulator::action::Action;
use hlt::direction::Direction;
use simulator::logger::log;
use simulator::memory::Memory;

//TODO Description
/// The state of the game at a certain turn.
/// Based on this data, a Bot decides what to do.
pub struct TurnState {
    turn_number: u32,

    // Amount of halite in each cell.
    // access with [y][x]
    halite_map: Vec<Vec<u16>>,
    /// The shipyard is also a dropoff
    dropoffs_pos: Vec<Position>,
    ships: HashMap<ShipId, Ship>,
    //other_ships: Probability map

    /// # Overwrites:
    /// The following values take priority over the above.
    /// They are the undoable state changes.

    /// Only the halite in a few cells is collected.
    /// Copying the entire halite_map would be expensive.
    overwrite_cells: HashMap<Position, u16>,
    overwrite_ships: HashMap<ShipId, Ship>,
}

impl TurnState {

    /// # Game Logic
    /// A bot did action on the previous turn which
    /// has an effect on this turn.
    pub fn did_action(&mut self, action: Action) {
        //log(&format!("Next turn action {{:?}}, overwrite_ships {}, overwrite_cells {}",
        //    self.overwrite_ships.len(), self.overwrite_cells.len()));
        match action {
            Action::MoveShip(id, direction) =>
                self.move_ship(id, direction),
            Action::None => {}
        }
    }

    fn move_ship(&mut self, id: ShipId, direction: Direction) {
        let mut ship = self.ship(id).clone();
        let mut real_direction = direction;
        // Move
        if direction != Direction::Still {
            let before = self.halite_at(&ship.position) as usize;
            let cost = before / 10;
            // Enough fuel
            if cost <= ship.halite {
                ship.halite -= cost;
                ship.position = ship.position.directional_offset(direction);
            } else {
                // Not enough: stand still.
                real_direction = Direction::Still;
            }
        }
        // Either stand still or not enough fuel.
        if real_direction == Direction::Still {
            let before = self.halite_at(&ship.position);
            let collect = max_collect(before);
            //TODO calculate halite when ship becomes full.

            ship.halite += collect as usize;
            self.overwrite_cells.insert(ship.position, before - collect);
        }
        
        self.overwrite_ships.insert(id, ship);
    }

    /// # Helpers and Getters

    /// Get a ship either from overwrite_ships or ships.
    /// Panics if the ship doesn't exist.
    pub fn ship(&self, id: ShipId) -> &Ship {
        match self.overwrite_ships.get(&id) {
            Some(ship) => ship,
            None => self.ships.get(&id).unwrap()
        }
    }

    pub fn halite_at(&self, pos: &Position) -> u16 {
        match self.overwrite_cells.get(pos) {
            Some(halite) => halite.clone(),
            None => at_normalized(&self.halite_map, pos)
        }
    }
    
    /// Check if cell is occupied.
    // TODO Optimize: HashMap<Position, ShipId>
    // But that is redundant data.
    pub fn ship_at(&self, pos: Position) -> Option<ShipId> {
        // Iterate through all ships and find the one with the given position.
        // Asserts that all overwrite_ships are in ships.
        for (id, ship) in &self.ships {
            let position =
                if let Some(overwrite) = self.overwrite_ships.get(id) {
                    overwrite.position
                } else {
                    ship.position
                };
            
            // If found, jump out of loop.
            if pos == position {
                return Some(id.clone());
            }
        }
        None
    }

    pub fn dropoff_near(&self, id: ShipId) -> Position {
        self.dropoffs_pos[0].clone() //TODO find nearest.
    }

    /// # State and Rollback management

    /// Based on the actual game.
    pub fn new_current(hlt_game: &Game) -> TurnState {
        TurnState {
            turn_number: hlt_game.turn_number as u32,
            halite_map: hlt_game.game_map.get_halite_map(),
            dropoffs_pos: my_shipyard_and_dropoff_positions(hlt_game),
            ships: hlt_game.ships.clone(),
            overwrite_cells: HashMap::new(),
            overwrite_ships: HashMap::new(),
        }
    }

    pub fn new_next(previous: &TurnState, memory: &Memory) -> TurnState {
        let turn_number = previous.turn_number + 1;
        let mut turn = TurnState {
            turn_number,
            halite_map: previous.halite_map.clone(),
            dropoffs_pos: previous.dropoffs_pos.clone(),
            ships: previous.ships.clone(),
            overwrite_cells: memory.cell_ow_on(turn_number),
            overwrite_ships: memory.ship_ow_on(turn_number),
        };
        // Moves that were already decided by other bots
        // in previous turns.
        turn.write_cells_and_ships();
        turn.overwrite_cells.clear();
        turn.overwrite_ships.clear();
        
        turn
    }

    /// Let this turn know of previous' overwrites.
    pub fn clone_overwrites_from(&mut self, previous: &TurnState) {
        self.overwrite_cells = previous.overwrite_cells.clone();
        self.overwrite_ships = previous.overwrite_ships.clone();
    }

    /// Clear overwrites.
    pub fn rollback(&mut self) {
        self.overwrite_cells.clear();
        self.overwrite_ships.clear();
    }
    /// Apply decided actions (overwrites), so other bots know
    /// of them. Mutates the halite_map and ships.
    pub fn apply(&mut self, memory: &Memory) {
        memory.store_cell_ow(self.turn_number, self.overwrite_cells.clone());
        memory.store_ship_ow(self.turn_number, self.overwrite_ships.clone());
        
        self.write_cells_and_ships();
        
        self.overwrite_cells.clear();
        self.overwrite_ships.clear();
    }
    fn write_cells_and_ships(&mut self) {
        for (pos, halite) in &self.overwrite_cells {
            // Write at the location of the cell pointer.
            * at_normalized_mut(&mut self.halite_map, pos) = *halite;
        }
        for (id,ship) in self.overwrite_ships.drain() {
            self.ships.insert(id, ship);
        }
    }
}
/// Get a list of positions where ships can deposit their cargo.
fn my_shipyard_and_dropoff_positions(hlt_game: &Game) -> Vec<Position> {
    let me = &hlt_game.players[hlt_game.my_id.0];
    let dropoff_ids = &me.dropoff_ids;
    let mut shipyard_dropoffs = Vec::with_capacity(dropoff_ids.len() +1);

    for id in dropoff_ids {
        if let Some(dropoff) = hlt_game.dropoffs.get(id) {
            shipyard_dropoffs.push(dropoff.position);
        }
    }
    shipyard_dropoffs.push(me.shipyard.position);
    return shipyard_dropoffs
}

/// Return a pointer a cell.
/// Wrap around the edge of the map. Copied from game_map.rs
fn at_normalized_mut<'m>(map: &'m mut Vec<Vec<u16>>, pos: &Position) -> &'m mut u16 {
    assert_ne!(map.len(), 0);
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    &mut map[(((pos.y % height) + height) % height) as usize][(((pos.x % width) + width) % width) as usize]
}
/// Wrap around the edge of the map. Copied from game_map.rs
fn at_normalized(map: &Vec<Vec<u16>>, pos: &Position) -> u16 {
    assert_ne!(map.len(), 0);
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    map[(((pos.y % height) + height) % height) as usize][(((pos.x % width) + width) % width) as usize]
}

/// This number can be substracted from the halite aount in thr cell.
//fixme: What happens when ship is full?
/// Collect: 25% of halite available in cell,
/// rounded up to the nearest whole number.
fn max_collect(halite_in_cell: u16) -> u16 {
    if halite_in_cell >= 1 {
        (halite_in_cell +3) /4
    } else {
        // No negative halite
        0
    }
}





#[cfg(test)]
mod test {
    use simulator::turn_state::TurnState;
    use hlt::position::Position;
    use std::collections::HashMap;
    use simulator::turn_state::at_normalized_mut;
    use simulator::turn_state::at_normalized;
    use hlt::ship::test::sample_ship;
    use simulator::action::Action;
    use hlt::direction::Direction;
    use simulator::turn_state::max_collect;

    pub fn create_test_data() -> TurnState {
        TurnState {
            turn_number: 42,
            halite_map: test_map(),
            dropoffs_pos: vec![Position{x: 12, y: 12}],
            ships: HashMap::new(), //TODO test-ship
            overwrite_cells: HashMap::new(),
            overwrite_ships: HashMap::new(),
        }
    }

    const TEST_HALITE_AMOUNT: u16 = 99;

    fn test_map() -> Vec<Vec<u16>> {
        let width = 48;
        let height = 48;

        let mut halite_map: Vec<Vec<u16>> = Vec::with_capacity(height);
        for _ in 0..height {
            let mut halite_row: Vec<u16> = Vec::with_capacity(width);
            for _ in 0.. width {
                let halite = TEST_HALITE_AMOUNT;
                halite_row.push(halite);
            }
            halite_map.push(halite_row);
        }
        halite_map
    }

    //TODO Correct halite calculation
    #[test]
    fn did_action_and_get_halite_and_ship() {
        let mut turn_state = create_test_data();
        let ship = sample_ship(Position{x: 3, y: 19});
        let action = Action::MoveShip(ship.id, Direction::North);
        turn_state.ships.insert(ship.id, ship.clone());

        let ship_still = sample_ship(Position{x: 31, y: 41});
        let action2 = Action::MoveShip(ship_still.id, Direction::Still);
        turn_state.ships.insert(ship_still.id, ship_still.clone());

        turn_state.did_action(action);
        turn_state.did_action(action2);
        // Ship moved correctly
        assert_eq!(turn_state.ship(ship.id).position, Position{x:3,y:18});
        assert_eq!(turn_state.ship(ship_still.id).position, Position{x:31,y:41});
        // Halite was collected
        assert_eq!(turn_state.halite_at(&ship_still.position),
            TEST_HALITE_AMOUNT - max_collect(TEST_HALITE_AMOUNT));
    }

    #[test]
    fn multiple_actions() {
        let mut turn_state = create_test_data();
        let ship = sample_ship(Position{x: 50, y: 50});
        turn_state.ships.insert(ship.id, ship.clone());

        turn_state.did_action(Action::MoveShip(ship.id, Direction::East));
        assert_eq!(turn_state.ship(ship.id).position, Position{x:51,y:50});
        assert_eq!(turn_state.halite_at(&Position{x:51,y:50}), TEST_HALITE_AMOUNT);

        turn_state.did_action(Action::MoveShip(ship.id, Direction::South));
        assert_eq!(turn_state.ship(ship.id).position, Position{x:51,y:51});
        assert_eq!(turn_state.halite_at(&Position{x:51,y:50}), TEST_HALITE_AMOUNT);

        // Stay still and collect
        let pos = Position{x:51,y:51};
        let halite_before = turn_state.halite_at(&pos);
        turn_state.did_action(Action::MoveShip(ship.id, Direction::Still));
        // Stay still
        assert_eq!(turn_state.ship(ship.id).position, pos);
        // Now there is less than before.
        assert!(turn_state.halite_at(&pos) < halite_before);
    }

    #[test]
    fn at_normalized_test() {
        let mut map = test_map();
        let pos = Position{x: 10, y: 15};

        // set and read
        *at_normalized_mut(&mut map, &pos) = 1111;
        assert_eq!(at_normalized(&map, &pos), 1111);
        assert_eq!(map[pos.y as usize][pos.x as usize], 1111);
    }

    #[test]
    fn max_collect_test() {
        assert_eq!(max_collect(0), 0);
        assert_eq!(max_collect(1), 1);
        assert_eq!(max_collect(99), 25);
        //Collected real data
        assert_eq!(max_collect(174), 174-130);
        assert_eq!(max_collect(153), 153-114);
        assert_eq!(max_collect(195), 195-146);
        assert_eq!(max_collect(316), 316-237);
    }
}