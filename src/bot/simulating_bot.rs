use std::cell::RefCell;
use std::rc::Rc;

use hlt::command::Command;
use hlt::direction::Direction;
use hlt::log::Log;
use hlt::ShipId;
use simulator::memory::Memory;
use simulator::simulator::Simulator;
use simulator::action::Action;
use bot::random_path_generator::RandomPathGenerator;

pub struct SimulatingBot<'turn > {
    simulator: &'turn mut Simulator<'turn>,
    memory: &'turn Memory,
    logger: Rc<RefCell<Log>>,

    id: ShipId,
}

/// This bot decides the action of one bot. Future actions
/// are saved in Memory. The bot object must be reconstructed
/// every turn due to Ship lifetime.
impl<'turn> SimulatingBot<'turn> {

    pub fn new<'t>(
        id: ShipId,
        simulator: &'t mut Simulator<'t>,
        logger: Rc<RefCell<Log>>,
    ) -> SimulatingBot<'t> {
        SimulatingBot {
            simulator,
            memory: simulator.memory,
            logger,
            id,
        }
    }

    /// Do the AI.
    pub fn calculate_command(&mut self) -> Command {
        // Outline:
        // come up with a random path
        // come up with a good path back to a dropoff
        // simulate how much halite that would collect
        // repeat 10-100 times and take the best one.

        let dir: Direction;

        let mut path = self.memory.moves_of_ship(&self.id);
        if path.len() <= 0 {
            path = self.make_path();
            self.logger.borrow_mut().log(&format!("Path length: {}", path.len()))
        }
        // One movement per turn.
        dir = path.pop().unwrap();
        self.memory.store_moves(self.id, path);

        self.simulator.id_to_ship(self.id).move_ship(dir)
    }

    /// Move random until the ship is almost filled up.
    /// Then go to a dropoff.
    fn make_path(&mut self) -> Vec<Direction> {
        const MOVE_RANDOM_CARGO: usize = 700;
        let gen = RandomPathGenerator::new();
        let mut output = Vec::new();

        // Move random until the ship is partially filled up.
        while self.simulator.id_to_ship(self.id).halite <= MOVE_RANDOM_CARGO
            && output.len() < 10 // Fail-safe
        {
            let dir = if self.move_or_collect() {
                gen.random_move()
            } else { Direction::Still };

            output.push(dir);
            self.log(dir);
            let action = Action::MoveShip(self.id, dir);
            self.simulator.do_and_switch_to_next_turn(action);
        }

        // Then move until dropoff reached.
        let dropoff_pos = self.simulator.dropoff_near(self.id);
        while dropoff_pos != self.simulator.id_to_ship(self.id).position
            && output.len() < 20
        {
            let dir = if self.move_or_collect() {
                self.simulator.navigate(self.id, &dropoff_pos)
            } else { Direction::Still };

            output.push(dir);
            self.log(dir);
            let action = Action::MoveShip(self.id, dir);
            self.simulator.do_and_switch_to_next_turn(action);
        }

        output
    }

    /// Returns true if the ship should move,
    /// or false if it should collect.
    fn move_or_collect(&self) -> bool {
        // If a cell contains less than this, it is considered empty.
        const CELL_EMPTY: u16 = 20;
        let ship = self.simulator.id_to_ship(self.id);
        let sim = &self.simulator;

        if ship.halite < (sim.halite_at(&ship.position) /10) as usize {
            // if not enough fuel, stay still
            false
        } else if sim.halite_at(&ship.position) <= CELL_EMPTY
            || ship.is_full() {
            // cell almost empty or ship full, move
            true
        } else {
            // cell has halite and ship can collect.
            false
        }
    }

    fn log(&self, dir: Direction) {
        let ship = self.simulator.id_to_ship(self.id);
        self.logger.borrow_mut().log(&format!(
            "ship: {}, map: {}, pos: {} {}, move: {:?}", ship.halite,
            self.simulator.halite_at(&ship.position),
            ship.position.x, ship.position.y, dir));
    }
}



#[cfg(test)]
mod tests {

}