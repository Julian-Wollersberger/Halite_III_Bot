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
    pub fn calculate(&mut self) -> Command {
        // Outline:
        // come up with a random path
        // come up with a good path back to a dropoff
        // simulate how much halite that would collect
        // repeat 10-100 times and take the best one.

        let dir: Direction;

        let mut path = self.memory.moves_of_ship(&self.id);
        if path.len() <= 0 {
            path = self.path();
            self.logger.borrow_mut().log(&format!("Step number: {}", path.len())[..])
        }
        // One movement per turn.
        dir = path.pop().unwrap();
        self.memory.store_moves(self.id, path);

        self.simulator.id_to_ship(self.id).move_ship(dir)
    }

    /// Move random until the ship is almost filled up.
    /// Then go to a dropoff.
    fn path(&mut self) -> Vec<Direction> {
        const MOVE_RANDOM_CARGO: usize = 700;
        let mut output = Vec::new();
        let gen = RandomPathGenerator::new();

        // Move random until the ship is partially filled up.
        while self.simulator.id_to_ship(self.id).halite <= MOVE_RANDOM_CARGO
            && output.len() < 10 // Fail-safe
        {
            let dir = self.decide_direction(&gen);

            output.push(dir);
            let action = Action::MoveShip(self.id, dir);
            self.simulator.do_and_switch_to_next_turn(action)
        }
        
        
        output
    }

    /// Comes up with a random direction,
    /// or if the ship should collect.
    // TODO or a generator that navigates to a dropoff. Lambda?
    fn decide_direction(&self, gen: &RandomPathGenerator) -> Direction{
        // If a cell contains less than this, it is considered empty.
        const CELL_EMPTY: u16 = 20;
        let ship = self.simulator.id_to_ship(self.id);
        let sim = &self.simulator;

        self.logger.borrow_mut().log(&format!(
            "ship: {}, map: {}, pos: {} {}", ship.halite,
            sim.halite_at(&ship.position),
            ship.position.x, ship.position.y));

        // if not enough fuel, stay still
        if ship.halite < (sim.halite_at(&ship.position) /10) as usize {
            Direction::Still

        // cell almost empty or ship full, move
        } else if sim.halite_at(&ship.position) <= CELL_EMPTY
            || ship.is_full()
        {
            gen.random_move()

        // cell has halite and ship can collect.
        } else {
            Direction::Still
        }
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn random_path_test() {
    }
}