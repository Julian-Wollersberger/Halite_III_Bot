use std::collections::HashMap;
use std::prelude::v1::Vec;

use hlt::game::Game;
use hlt::position::Position;
use hlt::ship::Ship;
use hlt::ShipId;
use simulator::action::Action;

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
    overwrite_cells: Vec<(Position, u16)>,
    overwrite_ships: HashMap<ShipId, Ship>,
}

impl TurnState {

    /// # Game Logic
    /// A bot did action on the previous turn which
    /// has an effect on this turn.
    pub fn did_action(&mut self, action: Action) {
        match action {
            Action::MoveShip(id, direction) => {
                //TODO calculate halite.
                let mut ship = self.ship(&id).clone();
                ship.position = ship.position.directional_offset(direction);
                ship.halite += 100; //Placeholder

                self.overwrite_cells.push((ship.position, 0));
                self.overwrite_ships.insert(id, ship);
            }
            Action::None => {}
        }
    }

    /// Get a ship either from overwrite_ships or ships.
    /// Panics if the ship doesn't exist.
    fn ship(&self, id: &ShipId) -> &Ship {
        match self.overwrite_ships.get(id) {
            Some(ship) => ship,
            None => self.ships.get(id).unwrap()
        }
    }



    /// # State and Rollback management

    /// Based on the actual game.
    pub fn new_current(hlt_game: &Game) -> TurnState {
        TurnState {
            turn_number: hlt_game.turn_number as u32,
            halite_map: hlt_game.game_map.get_halite_map(),
            dropoffs_pos: my_shipyard_and_dropoff_positions(hlt_game),
            ships: hlt_game.ships.clone(),
            overwrite_cells: Vec::new(),
            overwrite_ships: HashMap::new(),
        }
    }

    pub fn new_next(previous: &TurnState) -> TurnState {
        TurnState {
            turn_number: previous.turn_number + 1,
            halite_map: previous.halite_map.clone(),
            dropoffs_pos: previous.dropoffs_pos.clone(),
            ships: previous.ships.clone(),
            overwrite_cells: Vec::new(),
            overwrite_ships: HashMap::new()
        }
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
    pub fn apply(&mut self) {
        for (pos, halite) in &self.overwrite_cells {
            // Write at the location of the cell pointer.
            * at_normalized(&mut self.halite_map, pos) = *halite;
        }
        for (id,ship) in self.overwrite_ships.drain() {
            self.ships.insert(id, ship);
        }
        self.overwrite_cells.clear();
        self.overwrite_ships.clear();
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
fn at_normalized<'m>(map: &'m mut Vec<Vec<u16>>, pos: &Position) -> &'m mut u16 {
    assert_ne!(map.len(), 0);
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    &mut map[(((pos.y % height) + height) % height) as usize][(((pos.x % width) + width) % width) as usize]
}