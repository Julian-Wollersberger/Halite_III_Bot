use hlt::game::Game;
use simulator::action::Action;
use simulator::memory::Memory;
use simulator::turn_state::TurnState;

/// Be able to calculate the outcome of actions a few turns ahead.
/// I'm trying to not copy the gamefield data more than once per turn.
pub struct Simulator<'turn > {
    hlt_game: &'turn Game,
    memory: &'turn Memory,

    /// TODO Or use LinkedList? (because Vec::push might
    /// have to reallocate a lot of memory)
    future_turns: Vec<TurnState>,

    current_turn_index: usize,
}

impl<'turn > Simulator<'turn > {
    pub fn new(hlt_game: &'turn Game, memory: &'turn mut Memory) -> Simulator<'turn > {
        Simulator {
            hlt_game,
            memory,
            future_turns: vec![TurnState::new_current(hlt_game)],
            current_turn_index: 0,
        }
    }

    /// The action has an effect in the next turn.
    /// It's hard to seperate doing the action and
    /// switching to the next turn, because how can
    /// you then correctly tell the next turn the
    /// current turns's overwrite_* state?
    pub fn do_and_switch_to_next_turn(&mut self, action: Action) {
        // First, let next turn know of current's overwrites.

        //self.next().clone_overwrites_from(self.current())
        //This unsafe code is more elegant than doing the manipulations
        //outside of turn_state or having many setters.
        //https://stackoverflow.com/questions/52709147/how-to-workaround-the-coexistence-of-a-mutable-and-immutable-borrow
        let current: *const TurnState = self.current();
        unsafe {
            // Compiler doesn't know that current() and next() are separate.
            self.next().clone_overwrites_from(&*current)
        }

        //TODO Second, apply the action to the next turn.

        // Third, switch to next turn.
        self.next(); //Side effect: initialize next.
        self.current_turn_index += 1;
    }

    /// clear the changes made by actions since the last apply()
    pub fn rollback(&'turn mut self) {
        for turn in self.future_turns.iter_mut() {
            turn.rollback();
        }
    }
    /// If a bot has decided on its actions, it must apply them.
    /// Then bots processed later know of the actions and their effects on the map.
    pub fn apply(&mut self) {
        for turn in self.future_turns.iter_mut() {
            turn.apply();
        }
    }


    /// Get current turn
    pub fn current(&self) -> &TurnState{
        &self.future_turns[self.current_turn_index]
    }
    /// Get or init next turn
    pub fn next(&mut self) -> &mut TurnState {
        // init next if non-existent
        if self.future_turns.get(self.current_turn_index +1).is_none() {
            let next = TurnState::new_next(self.current());
            self.future_turns.push(next)
        }
        &mut self.future_turns[self.current_turn_index +1]
    }


    /*/// The ship should be a clone.
    pub fn collect_fast(&mut self, mut ship: Ship) {
    }
    pub fn calc_path_revenue(&self, path: Vec<Direction>) {
    } */
}