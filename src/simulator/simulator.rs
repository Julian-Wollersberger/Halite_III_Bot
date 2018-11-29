use hlt::game::Game;
use simulator::action::Action;
use simulator::memory::Memory;
use simulator::turn_state::TurnState;
use hlt::ShipId;
use hlt::ship::Ship;
use hlt::position::Position;
use hlt::direction::Direction;
use simulator::logger::log;
use simulator::Halite;
use rand::Rng;

/// Be able to calculate the outcome of actions a few turns ahead.
/// I'm trying to not copy the gamefield data more than once per turn.
pub struct Simulator<'turn > {
    hlt_game: &'turn Game,
    pub memory: &'turn Memory,
    
    future_turns: Vec<TurnState>,
    current_turn_index: usize,
}

impl<'turn > Simulator<'turn > {
    pub fn new(hlt_game: &'turn Game, memory: &'turn mut Memory) -> Simulator<'turn > {
        let mut sim = Simulator {
            hlt_game,
            memory,
            future_turns: vec![TurnState::new_current(hlt_game, memory)],
            current_turn_index: 0,
        };
        sim.next();
        sim
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

        //Second, apply the action to the next turn.
        self.next().did_action(action);

        // Third, switch to next turn.
        self.next(); //Side effect: initialize next.
        self.current_turn_index += 1;
        //Restore ships from memory, so is_safe() works.
        self.next();
    }

    /// clear the changes made by actions since the last apply()
    /// TODO partial rollback: only one/a few turns
    pub fn rollback(&mut self) {
        for turn in self.future_turns.iter_mut() {
            turn.rollback();
        }
        self.current_turn_index = 0;
    }
    /// If a bot has decided on its actions, it must apply them.
    /// Then bots processed later know of the actions and their effects on the map.
    pub fn apply(&mut self) {
        for turn in self.future_turns.iter_mut() {
            turn.apply();
        }
        self.current_turn_index = 0;
    }
    
    pub fn safe(&self) {
        for turn in self.future_turns.iter() {
            turn.save(self.memory);
        }
    }

    /// Get current turn
    #[inline]
    fn current(&self) -> &TurnState{
        &self.future_turns[self.current_turn_index]
    }
    /// Get or init next turn
    #[inline]
    fn next(&mut self) -> &mut TurnState {
        // init next if non-existent
        if self.future_turns.get(self.current_turn_index +1).is_none() {
            let next = TurnState::new_next(self.current(), &self.memory);
            self.future_turns.push(next)
        }
        &mut self.future_turns[self.current_turn_index +1]
    }

    pub fn id_to_ship(&self, id: ShipId) -> &Ship {
        self.current().ship(id)
    }
    pub fn halite_at(&self, pos: &Position) -> Halite {
        self.current().halite_at(pos)
    }
    pub fn dropoff_near(&self, id: ShipId) -> Position {
        self.hlt_game.me().shipyard.position
    }
    
    #[inline]
    /// Directions that would move the ship closer to the destination.
    pub fn useful_directions(&self, src: &Position, dst: &Position) -> Vec<Direction> {
        self.hlt_game.game_map.get_unsafe_moves(src, dst)
    }
    
    /// Will a ship be there in the next turn?
    pub fn is_safe(&self, dest: Position) -> bool { //fixme
        // Immutable next()
        if let Some(turn) = self.future_turns.get(self.current_turn_index +1) {
            // No ship there?
            let erg = turn.ship_at(dest).is_none();
            //log(&format!("Ship at {:?} on turn {}? {}", dest, self.current_turn_index, !erg));
            
            if rand::thread_rng().gen_bool(0.5) {
                erg
            } else { // Workaround: Ignore safety sometimes anyway.
                true
            }
        } else {
            // next() doesn't exist and moves
            // are not restored from memory.
            panic!("simulator.next() not initialised yet :(");
        }
    }
}