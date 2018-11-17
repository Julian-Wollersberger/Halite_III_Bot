extern crate core;
extern crate rand;

use std::env;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use hlt::game::Game;
use bot::simulate_future;

mod hlt;
mod simulator;
mod bot;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rng_seed: u64 = if args.len() > 1 {
        args[1].parse().unwrap()
    } else {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    };

    let game = Game::new();
    // At this point "game" variable is populated with initial map data.
    // This is a good place to do computationally expensive start-up pre-processing.
    // As soon as you call "ready" function below, the 2 second per turn timer will start.
    Game::ready("Julius-Beides");

    game.log.borrow_mut().log(&format!("Successfully created bot! My Player ID is {}. Bot rng seed is {}.", game.my_id.0, rng_seed));

    //fixed_pattern_bot::run(game);
    //move_random_and_back::run(game);
    //overseer::run(game);
    simulate_future::run(game);
}

/*
Move (n, s, e w) | Cost: 10% of halite available at
turn origin cell is deducted from ship’s current halite.

Move (o) | Collect: 25% of halite available in cell,
rounded up to the nearest whole number.
*/