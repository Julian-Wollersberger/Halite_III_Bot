use hlt::position::Position;
use hlt::game::Game;
use std::collections::HashMap;
use hlt::ShipId;
use hlt::ship::Ship;
use std::prelude::v1::Vec;
use core::borrow::BorrowMut;

/// The state of the game at a certain turn.
/// Prediction of a future turn.
/// Based on this data, a Bot decides what to do.
/// It's a linked list.
pub struct TurnState {
    // Lazily initialised Linked List
    next: Option<Box<TurnState>>,
    turn_number: u32,

    // Amount of halite in each cell
    halite_map: Vec<Vec<u16>>,
    /// The shipyard is also a dropoff
    dropoffs_pos: Vec<Position>,
    ships: HashMap<ShipId, Ship>,
    //other_ships: Probability map

    // These take priority over the above values.

    // Only the halite in a few cells is collected.
    // Copying the entire map would be expensive.
    undo_map: Vec<(Position, u16)>,
    undo_ships: HashMap<ShipId, Ship>,
}

impl TurnState {

    /// Create the Linked List.
    pub fn new_current(hlt_game: &Game) -> TurnState {
        TurnState {
            next: None,
            turn_number: hlt_game.turn_number as u32,
            halite_map: hlt_game.game_map.get_halite_map(),
            dropoffs_pos: my_shipyard_and_dropoff_positions(hlt_game),
            ships: hlt_game.ships.clone(),
            undo_map: Vec::new(),
            undo_ships: HashMap::new(),
        }
    }

    pub fn next(&self) -> Option<&TurnState> {
        match &self.next {
            Some(t) => Some(&t),
            None => None,
        }
    }

    /// Get or create next TurnState
    pub fn next_or_create<'a>(&'a mut self) -> &'a mut TurnState {
        if self.next.is_some() {
            &mut self.next.unwrap()
        } else {
            self.next = Some(TurnState::create_next(self));
            &mut self.next.unwrap()
        }
    }

    fn create_next(previous: &TurnState) -> Box<TurnState> {
        /* https://stackoverflow.com/questions/35201250/what-is-the-difference-between-using-the-box-keyword-and-boxnew#35201819
        box is magic and made up ground-up pixies and the
        dreams of little children. It is dressed in the finest,
        swankiest clothes and carries with it the faint aroma
        of freshly cut pine. */
        Box::new(TurnState {
            next: None,
            turn_number: previous.turn_number + 1,
            halite_map: previous.halite_map.clone(),
            dropoffs_pos: previous.dropoffs_pos.clone(),
            ships: previous.ships.clone(),
            // TODO or clone previous?
            undo_map: Vec::new(),
            undo_ships: HashMap::new()
        })
    }
}

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
