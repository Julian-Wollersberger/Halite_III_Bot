extern crate core;
extern crate rand;

#[macro_use]
extern crate serde_derive;

use std::env;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use hlt::game::Game;
use bot::simulate_future;
use simulator::logger::log;
use simulator::logger::set_logger;

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
    let start_time = SystemTime::now();
    
    let game = Game::new();
    // At this point "game" variable is populated with initial map data.
    // This is a good place to do computationally expensive start-up pre-processing.
    // As soon as you call "ready" function below, the 2 second per turn timer will start.
    Game::ready("Julius-Beides");

    set_logger(game.log.clone());
    log("Successfully initialised global logger!");
    log(&format!("Successfully created bot! \
        My Player ID is {}.", game.my_id.0));
    
    //fixed_pattern_bot::run(game);
    //move_random_and_back::run(game);
    //overseer::run(game);
    simulate_future::run(game, start_time);
}

/*
Move (n, s, e w) | Cost: 10% of halite available at
turn origin cell is deducted from ship’s current halite.

Move (o) | Collect: 25% of halite available in cell,
rounded up to the nearest whole number.
*/