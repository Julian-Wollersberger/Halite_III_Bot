use hlt::game::Game;
use simulator::memory::Memory;
use simulator::turn_state::TurnState;
use simulator::action::Action;
use std::mem;

/// Be able to calculate the outcome of actions a few turns ahead.
/// I'm trying to not copy the gamefield data more than once per turn.
pub struct Simulator<'turn > {
    hlt_game: &'turn Game,
    memory: &'turn Memory,

    // Updates on nextTurn
    // Shortcut to an element in the LinkedList
    current_turn: &'turn mut TurnState,

    // List of future_turns: TurnState
    real_turn: TurnState,
}

impl<'turn > Simulator<'turn > {
    pub fn new(hlt_game: &'turn Game, memory: &'turn mut Memory) -> Simulator<'turn > {
        let mut real_turn = TurnState::new_current(hlt_game);

        unsafe {
            let mut sim = Simulator {
                hlt_game,
                memory,
                // definitely lives longs enough, but moved.
                current_turn: mem::uninitialized(),
                real_turn,
            };
            sim.current_turn = &mut sim.real_turn;

            sim
        }
    }

    fn next_turn(&'turn mut self) {
        //self.current_turn = self.current_turn.next_or_create();
        mem::replace(&mut self.current_turn, self.current_turn.next_or_create());
    }

    /// do an action
    fn doo(&mut self, action: Action) {

    }

    /// clear the changes made by actions since the last apply()
    fn rollback(&'turn mut self) {
        self.current_turn = &mut self.real_turn;
    }
    /// If a bot has decided on its actions, it must apply them.
    /// Then bots processed later know of the actions and their effects on the map.
    fn apply(&mut self) {

    }


    /*/// The ship should be a clone.
    pub fn collect_fast(&mut self, mut ship: Ship) {
    }
    pub fn calc_path_revenue(&self, path: Vec<Direction>) {
    } */
}