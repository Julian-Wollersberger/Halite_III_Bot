extern crate rand;

use std::cell::RefCell;
use std::rc::Rc;
use rand::Rng;

use hlt::command::Command;
use hlt::direction::Direction;
use hlt::log::Log;
use hlt::ShipId;
use simulator::memory::Memory;
use simulator::simulator::Simulator;
use hlt::ship::Ship;
use simulator::action::Action;

pub struct SimulatingBot<'turn > {
    simulator: &'turn mut Simulator<'turn>,
    memory: &'turn Memory,
    logger: Rc<RefCell<Log>>,

    id: ShipId,
    //ship: &'turn Ship,
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
            //ship: simulator.id_to_ship(id),
        }
    }

    /// Do the AI.
    pub fn calculate(&mut self) -> Command {
        // Outline:
        // come up with a random path
        // come up with a good path back to a dropoff
        // simulate how much halite that would collect
        // repeat 10-100 times and take the best one.

        let mut dir: Direction;

        let mut path = self.memory.moves_of_ship(&self.id);
        if path.len() <= 0 {
            path = self.optimize_path(
                random_path(5, 15));
            self.logger.borrow_mut().log(&format!("Step number: {}", path.len())[..])
        }
        dir = path.pop().unwrap();
        self.memory.store_moves(self.id, path);

        if dir == Direction::Still {
            self.logger.borrow_mut().log(&format!("Collecting."));
        }

        //self.ship.move_ship(dir)
        self.simulator.id_to_ship(self.id).move_ship(dir)
    }

    /// Puts Direction::Still in between of movements.
    /// TODO shorten: Dismiss the remaining movements if ship gets full.
    /// Returns an empty Vec if the destination is reached.
    fn optimize_path(&mut self, mut path: Vec<Direction>) -> Vec<Direction> {
        // If a cell contains less than this, it is considered empty.
        const CELL_EMPTY: u16 = 100;
        let mut output = Vec::new();

        //TODO Define iteration order.
        while path.len() > 0
            && output.len() < 100 // Fail-safe
        {
            let action = {
                let ship = self.simulator.id_to_ship(self.id);
                let sim = &self.simulator;
                self.logger.borrow_mut().log(&format!(
                    "ship: {}, map: {}, pos: {} {}", ship.halite,
                    sim.halite_at(&ship.position),
                    ship.position.x, ship.position.y));

                // if not enough fuel, stay still
                let dir = if ship.halite < (sim.halite_at(&ship.position) /10) as usize {
                    Direction::Still
                }
                // cell almost empty or ship full, move
                else if sim.halite_at(&ship.position) < CELL_EMPTY
                    || ship.is_full()
                {
                    path.pop().unwrap()
                }
                // cell has halite and ship can collect.
                else {
                    Direction::Still
                };
                output.push(dir);
                Action::MoveShip(self.id, dir)
            };

            self.simulator.do_and_switch_to_next_turn(action)
        }
        output
    }

}

/// make a list of moves that don't backtrack or stand still.
/// Optimal collection turns are calculated later by the simulator.
fn random_path(min_steps: i32, max_steps: i32) -> Vec<Direction> {
    // Make sure to only move in one quadrant.
    // Don't let random movement cancel itself out.
    let vertical_direction =
        if rand::thread_rng().gen_bool(0.5) {
            Direction::North
        } else { Direction::South };

    let horizontal_direction =
        if rand::thread_rng().gen_bool(0.5) {
            Direction::East
        } else { Direction::West };

    let num_steps = rand::thread_rng().gen_range(min_steps,max_steps);
    let mut directions: Vec<Direction> = Vec::with_capacity(num_steps as usize);

    for _ in 0..num_steps {
        // Either go horizontally or vertically.
        if rand::thread_rng().gen_bool(0.5) {
            directions.push(vertical_direction)
        } else {
            directions.push(horizontal_direction)
        }
    }

    return directions
}


#[cfg(test)]
mod tests {
    use bot::simulating_bot::random_path;

    #[test]
    fn random_path_test() {
        for _ in 0..10 {
            let path = random_path(1, 5);
            assert!(path.len() >= 1);
            assert!(path.len() < 5);
        }
    }
}