extern crate rand;

use std::cell::RefCell;
use std::rc::Rc;

use hlt::command::Command;
use hlt::direction::Direction;
use hlt::log::Log;
use hlt::ShipId;
use simulator::memory::Memory;
use simulator::simulator::Simulator;
use simulator::action::Action;
use bot::path_finder::PathFinder;
use hlt::ship::Ship;
use rand::Rng;
use simulator::logger::log;
use simulator::Halite;

pub struct SimulatingBot<'turn > {
    simulator: &'turn mut Simulator<'turn>,
    memory: &'turn Memory,

    id: ShipId,
}

/// This bot decides the action of one bot. Future actions
/// are saved in Memory. The bot object must be reconstructed
/// every turn due to Ship lifetime.
impl<'turn> SimulatingBot<'turn> {

    pub fn new<'t>(
        id: ShipId,
        simulator: &'t mut Simulator<'t>,
    ) -> SimulatingBot<'t> {
        SimulatingBot {
            simulator,
            memory: simulator.memory,
            id,
        }
    }

    pub fn calculate_command(&mut self) -> Command {
        let dir: Direction;

        let mut path = self.memory.ship_path(&self.id);
        if path.len() <= 0 {
            path = self.calc_good_path();
            log(&format!("Path length: {}", path.len()))
        }
        // One movement per turn.
        dir = path.pop().unwrap();
        self.memory.store_path(self.id, path);

        self.simulator.id_to_ship(self.id).move_ship(dir)
    }

    /// Outline:
    /// come up with a random path
    /// come up with a good path back to a dropoff
    /// simulate how much halite that would collect
    /// repeat 10-100 times and take the best one.
    fn calc_good_path(&mut self) -> Vec<Direction> {
        let mut best_score = 0;
        let mut best_path = vec![Direction::Still];
        self.simulator.rollback();
        
        // Find the best out of some random ones.
        for i in 0..1 {
            let go_back_cargo = rand::thread_rng().gen_range(200, 800);
            let cell_empty = biased_range(10, 100) as Halite;
            let path = self.some_complete_path(go_back_cargo, cell_empty);
            
            // Simulator & ship changed state.
            let score = 10 * self.ship().halite / path.len();
            if score > 0 { log(&format!(
                "Path {}, len {}, go_back {}, cell_empty {} \
                would collect {} and score\t{}",
                    i, path.len(), go_back_cargo, cell_empty,
                    self.ship().halite, score));
            }
            
            if  score > best_score {
                best_score = score;
                best_path = path;
            }
            self.simulator.rollback();
        }
        
        // Apply and return the best one.
        log(&format!("Best score {}, path length {}", best_score, best_path.len()));
        self.replay_path(&best_path);
        self.simulator.apply();
        best_path
    }

    /// Move random until the ship is almost filled up.
    /// Then go to a dropoff.
    fn some_complete_path(&mut self, go_back_cargo: usize, cell_empty: Halite) -> Vec<Direction> {
        const MAX_PATH_LEN: usize = 200;
        let finder = PathFinder::new();
        let mut path = Vec::new();

        // Move random until the ship is partially filled up.
        while self.ship().halite <= go_back_cargo
            && path.len() < MAX_PATH_LEN /2 // Fail-safe
        {
            let dir = if self.move_or_collect(cell_empty) {
                //log("Move");
                finder.safe_random_move(self.ship(), self.simulator)
            } else { Direction::Still };

            path.push(dir);
            //self.log_ship(dir);
            let action = Action::MoveShip(self.id, dir);
            self.simulator.do_and_switch_to_next_turn(action);
        }

        // Then move until dropoff reached.
        let dropoff_pos = self.simulator.dropoff_near(self.id);
        while dropoff_pos != self.ship().position
            && path.len() < MAX_PATH_LEN
        {
            let dir = if self.move_or_collect(cell_empty) {
                finder.navigate_to_dest(&dropoff_pos,
                    self.ship(), self.simulator)
            } else { Direction::Still };

            path.push(dir);
            //self.log_ship(dir);
            let action = Action::MoveShip(self.id, dir);
            self.simulator.do_and_switch_to_next_turn(action);
        }

        // Invert the order.
        let mut output = Vec::with_capacity(path.len());
        for dir in path.into_iter().rev() {
            output.push(dir);
        }
        
        output
    }

    /// Returns true if the ship should move,
    /// or false if it should collect.
    fn move_or_collect(&self, cell_empty: Halite) -> bool {
        // If a cell contains less than this, it is considered empty.
        let ship = self.ship();
        let sim = &self.simulator;
        let pos = &ship.position;//.directional_offset(offset);

        if ship.halite < (sim.halite_at(pos) /10) as usize {
            // if not enough fuel, stay still
            false
        } else if sim.halite_at(pos) <= cell_empty
            || ship.is_full() {
            // cell almost empty or ship full, move
            true
        } else {
            // cell has halite and ship can collect.
            false
        }
    }
    /// Move the ship on the path to change the simulator's state.
    fn replay_path(&mut self, path: &Vec<Direction>) {
        for dir in path {
            let action = Action::MoveShip(self.id, dir.clone());
            self.simulator.do_and_switch_to_next_turn(action);
        }
    }

    fn log_ship(&self, dir: Direction) {
        let ship = self.simulator.id_to_ship(self.id);
        log(&format!(
            "ship: {}, map: {}, pos: {} {}, move: {:?}", ship.halite,
            self.simulator.halite_at(&ship.position),
            ship.position.x, ship.position.y, dir));
    }
    
    fn ship(&self) -> &Ship {
        self.simulator.id_to_ship(self.id)
    }
}

/// Returns a value between min and max
/// Where small values are more likely.
fn biased_range(min: u16, max: u16) -> u16 {
    // Quadratic
    //let sqrt = rand::thread_rng().gen_range((min as f64).sqrt(), (max as f64).sqrt());
    //(sqrt * sqrt) as u16
    
    /*// Exponential
    let min_exp = (min as f64).log2();
    let max_exp = (max as f64).log2();

    let exponent = rand::thread_rng().gen_range(min_exp, max_exp);
    2_f64.powf(exponent) as u16
    */
    // Linear. Defeats the purpose of this function...
    //rand::thread_rng().gen_range(min, max)
    
    50
}


#[cfg(test)]
mod tests {

}