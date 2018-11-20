extern crate rand;

use hlt::direction::Direction;
use rand::Rng;
use hlt::ship::Ship;
use simulator::simulator::Simulator;
use hlt::position::Position;
use simulator::logger::log;

/// Do moves that don't backtrack or stand still.
/// Make sure to only move in one quadrant.
/// Don't let random movement cancel itself out.
pub struct PathFinder {
    // for random moves.
    vertical_dir: Direction,
    horizontal_dir: Direction,
}

impl PathFinder {

    pub fn new() -> PathFinder {
        let vertical_dir =
            if rand::thread_rng().gen_bool(0.5) {
                Direction::North
            } else { Direction::South };

        let horizontal_dir =
            if rand::thread_rng().gen_bool(0.5) {
                Direction::East
            } else { Direction::West };

        PathFinder { vertical_dir, horizontal_dir}
    }
    
    pub fn safe_random_move(&self, ship: &Ship, simulator: &Simulator) -> Direction {
        let preferred = self.random_move();
        
        if simulator.is_safe(ship.position.directional_offset(preferred)) {
            //log(&format!("Preferred {:?}", preferred));
            preferred
        } else {
            let other =
                if preferred == self.horizontal_dir {
                    self.vertical_dir
                } else { self.horizontal_dir };
            if simulator.is_safe(ship.position.directional_offset(other)) {
                log(&format!("Other {:?}, Preferred {:?}", other, preferred));
                other
            } else {
                log(&format!("Still :( ,Other {:?}, Preferred {:?}", other, preferred));
                Direction::Still
            }
        }
    }

    /// Optimal collection turns are calculated later by the simulator.
    fn random_move(&self) -> Direction {
        // Either go horizontally or vertically.
        if rand::thread_rng().gen_bool(0.5) {
            self.vertical_dir.clone()
        } else {
            self.horizontal_dir.clone()
        }
    }
    
    pub fn navigate_to_dest(
        &self, dest: &Position, ship: &Ship, simulator: &Simulator
    ) -> Direction{
        // Safe and useful directions.
        let mut safe = Vec::new();
        
        for dir in simulator.useful_directions(&ship.position, dest) {
            if simulator.is_safe(ship.position.directional_offset(dir)) {
                safe.push(dir);
            }
        }
        // Random entry or still.
        if safe.len() >= 2 {
            safe[rand::thread_rng().gen_range(0, safe.len())]
        } else if safe.len() >= 1 {
            log(&format!("PF: One safe direction: {:?}", safe[0]));
            safe[0]
        } else {
            log(&format!("PF: Every path blocked. Dest {:?}, Ship {:?}", dest, ship.position));
            // Evade: Chose a random direction and try to move there.
            let dir = Direction::get_all_cardinals()[rand::thread_rng().gen_range(0, 4)];
            if simulator.is_safe(ship.position.directional_offset(dir)) {
                log(&format!("PF: Evade {:?}", dir));
                dir
            } else {
                Direction::Still
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use bot::path_finder::PathFinder;
    use std::collections::HashSet;

    #[test]
    fn only_two_directions() {
        let gen = PathFinder::new();
        let mut set = HashSet::new();

        for _ in 0..20 {
            // Ignores double entries.
            set.insert(gen.random_move());
        }
        assert_eq!(set.len(), 2)
    }
}
