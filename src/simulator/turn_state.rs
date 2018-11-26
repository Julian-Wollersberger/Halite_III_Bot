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
use std::rc::Rc;
use simulator::state_difference::StateDifference;
use simulator::Halite;
use core::mem;

/// The state of the game at a certain turn.
/// Based on this data, a Bot decides what to do.
pub struct TurnState {
    turn_number: u32,
    
    /// Amount of halite in each cell. Access with [y][x]
    real_halite_map: Rc<Vec<Vec<Halite>>>,
    real_ships: Rc<HashMap<ShipId, Ship>>,
    real_ship_pos: Rc<HashMap<Position, ShipId>>,
    
    /// Lowest priority: From Memory.
    saved_diff: StateDifference,
    applied_diff: StateDifference,
    /// Highest priority: Overwrites.
    undoable_diff: StateDifference,
}

impl TurnState {
    /// # Game Logic
    /// A bot did an action on the previous turn which
    /// has an effect on this turn.
    pub fn did_action(&mut self, action: Action) {
        //log(&format!("Next turn action {{:?}}, overwrite_ships {}, overwrite_cells {}",
        //    self.overwrite_ships.len(), self.overwrite_cells.len()));
        match action {
            Action::MoveShip(id, direction) => {
                log(&format!("Turn: did action Move({}, {:?})",
                    id.0, direction));
                self.move_ship(id, direction)
            },
            Action::None => {}
        }
    }
    
    fn move_ship(&mut self, id: ShipId, direction: Direction) {
        let old_ship = self.ship(id).clone();
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
            self.undoable_diff.set_halite(ship.position, before - collect);
        }
        
        self.undoable_diff.set_ship(ship, Some(&old_ship));
    }
    
    /// # Helpers and Getters
    
    /// Get a ship either from overwrite_ships or ships.
    /// Panics if the ship doesn't exist.
    pub fn ship(&self, id: ShipId) -> &Ship {
        self.undoable_diff.ship(id)
            .or_else(|| self.applied_diff.ship(id))
            .or_else(|| self.saved_diff.ship(id))
            .or_else(|| self.real_ships.get(&id))
            .expect("Ship does not exist in TurnState!")
    }
    fn overwrite_ship(&mut self, ship: Ship, old: &Ship) {
        self.undoable_diff.set_ship(ship, Some(old));
    }
    
    pub fn halite_at(&self, pos: &Position) -> Halite {
        if let Some(halite) = self.undoable_diff.halite(pos) {
            halite
        } else if let Some(halite) = self.applied_diff.halite(pos) {
            halite
        } else if let Some(halite) = self.saved_diff.halite(pos) {
            halite
        } else {
            at_normalized(&self.real_halite_map, pos)
        }
    }
    
    /// Check if cell is occupied.
    pub fn ship_at(&self, pos: Position) -> Option<ShipId> {
        if let Some(id) = self.undoable_diff.ship_at(pos) {
            Some(id)
        } else if let Some(id) = self.applied_diff.ship_at(pos) {
            Some(id)
        } else if let Some(id) = self.saved_diff.ship_at(pos) {
            Some(id)
        } else if let Some(id) = self.real_ship_pos.get(&pos) {
            Some(id.clone())
        } else {
            None
        }
    }
    
    /// # State and Rollback management
    
    /// Based on the actual game.
    pub fn new_current(hlt_game: &Game, memory: &Memory) -> TurnState {
        let turn_number = hlt_game.turn_number as u32;
        let real_ships = Rc::new(hlt_game.ships.clone());
        let mut ship_pos = HashMap::new();
        
        for (id, ship) in real_ships.iter() {
            ship_pos.insert(ship.position, id.clone());
        }
        
        TurnState{
            turn_number,
            real_halite_map: Rc::new(hlt_game.game_map.get_halite_map()),
            real_ships,
            real_ship_pos: Rc::new(ship_pos),
            saved_diff: memory.load_diff(turn_number),
            applied_diff: StateDifference::new(),
            undoable_diff: StateDifference::new(),
        }
    }

    pub fn new_next(previous: &TurnState, memory: &Memory) -> TurnState {
        TurnState{
            turn_number: previous.turn_number +1,
            real_halite_map: Rc::clone(&previous.real_halite_map),
            real_ships: Rc::clone(&previous.real_ships),
            real_ship_pos: Rc::clone(&previous.real_ship_pos),
            saved_diff: memory.load_diff(previous.turn_number +1),
            applied_diff: StateDifference::new(),
            undoable_diff: StateDifference::new(),
        }
    }
    
    pub fn clone_overwrites_from(&mut self, previous: &TurnState) {
        self.undoable_diff.clear();
        self.undoable_diff.extend(previous.undoable_diff.clone())
    }
    
    /// Clear overwrites.
    pub fn rollback(&mut self) {
        self.undoable_diff.clear()
    }
    /// Apply decided actions (overwrites), so other bots know
    /// of them.
    pub fn apply(&mut self) {
        let overwrites = mem::replace(
            &mut self.undoable_diff, StateDifference::new());
        self.applied_diff.extend(overwrites);
    }
    pub fn save(&self, memory: &mut Memory) {
        unimplemented!()
    }
}

/// Return a pointer a cell.
/// Wrap around the edge of the map. Copied from game_map.rs
fn at_normalized_mut<'m>(map: &'m mut Vec<Vec<Halite>>, pos: &Position) -> &'m mut Halite {
    assert_ne!(map.len(), 0);
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    &mut map[(((pos.y % height) + height) % height) as usize][(((pos.x % width) + width) % width) as usize]
}

/// Wrap around the edge of the map. Copied from game_map.rs
fn at_normalized(map: &Vec<Vec<Halite>>, pos: &Position) -> Halite {
    assert_ne!(map.len(), 0);
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    map[(((pos.y % height) + height) % height) as usize][(((pos.x % width) + width) % width) as usize]
}

/// This number can be substracted from the halite aount in thr cell.
//fixme: What happens when ship is full?
/// Collect: 25% of halite available in cell,
/// rounded up to the nearest whole number.
fn max_collect(halite_in_cell: Halite) -> Halite {
    if halite_in_cell >= 1 {
        (halite_in_cell + 3) / 4
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
    use std::rc::Rc;
    use simulator::Halite;
    use simulator::state_difference::StateDifference;
    
    pub fn create_test_data() -> TurnState {
        TurnState {
            turn_number: 42,
            real_halite_map: test_map(),
            real_ships: Rc::new(HashMap::new()),
            real_ship_pos: Rc::new(HashMap::new()),
            saved_diff: StateDifference::new(),
            applied_diff: StateDifference::new(),
            undoable_diff: StateDifference::new(),
        }
    }
    
    const TEST_HALITE_AMOUNT: u32 = 99;
    
    fn test_map() -> Rc<Vec<Vec<Halite>>> {
        let width = 48;
        let height = 48;
        
        let mut halite_map: Vec<Vec<Halite>> = Vec::with_capacity(height);
        for _ in 0..height {
            let mut halite_row: Vec<Halite> = Vec::with_capacity(width);
            for _ in 0..width {
                let halite = TEST_HALITE_AMOUNT;
                halite_row.push(halite);
            }
            halite_map.push(halite_row);
        }
        Rc::new(halite_map)
    }
    
    //TODO Correct halite calculation
    #[test]
    fn did_action_and_get_halite_and_ship() {
        let mut turn_state = create_test_data();
        let ship = sample_ship(Position { x: 3, y: 19 });
        let action = Action::MoveShip(ship.id, Direction::North);
        Rc::get_mut(&mut turn_state.real_ships).unwrap()
            .insert(ship.id, ship.clone());
        
        let ship_still = sample_ship(Position { x: 31, y: 41 });
        let action2 = Action::MoveShip(ship_still.id, Direction::Still);
        Rc::get_mut(&mut turn_state.real_ships).unwrap()
            .insert(ship_still.id, ship_still.clone());
        
        turn_state.did_action(action);
        turn_state.did_action(action2);
        // Ship moved correctly
        assert_eq!(turn_state.ship(ship.id).position, Position { x: 3, y: 18 });
        assert_eq!(turn_state.ship(ship_still.id).position, Position { x: 31, y: 41 });
        // Halite was collected
        assert_eq!(turn_state.halite_at(&ship_still.position),
            TEST_HALITE_AMOUNT - max_collect(TEST_HALITE_AMOUNT));
    }
    
    #[test]
    fn multiple_actions() {
        let mut turn_state = create_test_data();
        let ship = sample_ship(Position { x: 50, y: 50 });
        Rc::get_mut(&mut turn_state.real_ships).unwrap()
            .insert(ship.id, ship.clone());
        
        turn_state.did_action(Action::MoveShip(ship.id, Direction::East));
        assert_eq!(turn_state.ship(ship.id).position, Position { x: 51, y: 50 });
        assert_eq!(turn_state.halite_at(&Position { x: 51, y: 50 }), TEST_HALITE_AMOUNT);
        
        turn_state.did_action(Action::MoveShip(ship.id, Direction::South));
        assert_eq!(turn_state.ship(ship.id).position, Position { x: 51, y: 51 });
        assert_eq!(turn_state.halite_at(&Position { x: 51, y: 50 }), TEST_HALITE_AMOUNT);
        
        // Stay still and collect
        let pos = Position { x: 51, y: 51 };
        let halite_before = turn_state.halite_at(&pos);
        turn_state.did_action(Action::MoveShip(ship.id, Direction::Still));
        // Stay still
        assert_eq!(turn_state.ship(ship.id).position, pos);
        // Now there is less than before.
        assert!(turn_state.halite_at(&pos) < halite_before);
    }
    
    #[test]
    fn at_normalized_test() {
        let mut rc_map = test_map();
        let mut map = Rc::get_mut(&mut rc_map).unwrap();
        let pos = Position { x: 10, y: 15 };
        
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
        assert_eq!(max_collect(174), 174 - 130);
        assert_eq!(max_collect(153), 153 - 114);
        assert_eq!(max_collect(195), 195 - 146);
        assert_eq!(max_collect(316), 316 - 237);
    }
}