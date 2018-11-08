use hlt::game::Game;
use std::collections::HashMap;
use simulator::memory::Memory;
use simulator::turn_state::TurnNumber;
use simulator::turn_state::TurnState;
use hlt::ship::Ship;
use hlt::direction::Direction;

pub struct Simulator<'turn > {
    hlt_game: &'turn Game,
    memory: &'turn Memory,

    // List of future_turns: TurnState
    current_turn: TurnState,
}

impl<'turn > Simulator<'turn > {
    pub fn new(hlt_game: &'turn Game, memory: &'turn mut Memory) -> Simulator<'turn > {
        Simulator {
            hlt_game,
            memory,
            future_turns: HashMap::new()
        }
    }

    /// The ship should be a clone.
    pub fn collect_fast(&mut self, mut ship: Ship) {

    }

    pub fn calc_path_revenue(&self, path: Vec<Direction>)
}



/*
Recursive algorithm to calculate best move:

fn simulate(&mut self, turn)
    doAction(move)

    simulate(turn +1);

    undo()
}

With the undo the game doesn't have to be copied all the time.
undo:

self.collectAction.match {
 some => collectAction.undo()
}

*/


pub fn simulate_action() {

}

fn undo(count: i32) {
    //0: all
    //other: other
}

fn apply() {

}