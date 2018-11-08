use hlt::position::Position;
use hlt::game::Game;

/// The state of the game at a certain turn.
/// Prediction of a future turn.
/// Based on this data, a Bot decides what to do.
/// It's a linked list.
pub struct TurnState {
    // Lazily initialised Linked List
    next: Option<TurnState>,
    turn_number: TurnNumber,

    halite_map: Vec<Vec<u16>>,
    /// The shipyard is also a dropoff
    dropoffs_pos: Vec<Position>
    //my_ships
    //other_ships Probability map

    //undoable actions
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TurnNumber(pub u32);

impl TurnState {

    /// Create the Linked List.
    pub fn new_current(hlt_game: &Game) -> TurnState {
        TurnState {
            next: None,
            turn_number: TurnNumber(hlt_game.turn_number as u32),
            halite_map: hlt_game.game_map.get_halite_map(),
            dropoffs_pos: my_shipyard_and_dropoff_positions(hlt_game),
        }
    }

    fn create_next(previous: &TurnState) -> TurnState {
        TurnState {
            next: None,
            turn_number: previous.turn_number +1,
            halite_map: previous.halite_map.clone(),
            dropoffs_pos: previous.dropoffs_pos.clone(),
        }
    }

    /// Get or create next TurnState
    pub fn next(&mut self) -> &mut TurnState {
        if self.next.is_some() {
            &mut self.next.unwrap()
        } else {
            self.next = Some(TurnState::create_next(self));
            &mut self.next.unwrap()
        }
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
