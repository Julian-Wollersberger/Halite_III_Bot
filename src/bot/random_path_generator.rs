extern crate rand;

use hlt::direction::Direction;
use rand::Rng;

/// Do moves that don't backtrack or stand still.
/// Make sure to only move in one quadrant.
/// Don't let random movement cancel itself out.
pub struct RandomPathGenerator {
    vertical_dir: Direction,
    horizontal_dir: Direction,
}

impl RandomPathGenerator {

    pub fn new() -> RandomPathGenerator {
        let vertical_dir =
            if rand::thread_rng().gen_bool(0.5) {
                Direction::North
            } else { Direction::South };

        let horizontal_dir =
            if rand::thread_rng().gen_bool(0.5) {
                Direction::East
            } else { Direction::West };

        RandomPathGenerator { vertical_dir, horizontal_dir}
    }

    /// Optimal collection turns are calculated later by the simulator.
    pub fn random_move(&self) -> Direction {
        // Either go horizontally or vertically.
        if rand::thread_rng().gen_bool(0.5) {
            self.vertical_dir.clone()
        } else {
            self.horizontal_dir.clone()
        }
    }
}


#[cfg(test)]
mod tests {
    use bot::random_path_generator::RandomPathGenerator;
    use std::collections::HashSet;

    #[test]
    fn only_two_directions() {
        let gen = RandomPathGenerator::new();
        let mut set = HashSet::new();

        for _ in 0..20 {
            // Ignores double entries.
            set.insert(gen.random_move());
        }
        assert_eq!(set.len(), 2)
    }
}
