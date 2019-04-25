extern crate core;
extern crate rand;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

use std::time::SystemTime;
use hlt::game::Game;
use bot::simulate_future;
use simulator::logger::log;
use simulator::logger::set_logger;
use bot::simulate_future::deserialize_game;
use bot::simulate_future::run_loop;
use std::thread;

mod hlt;
mod simulator;
mod bot;

fn main() {
    start_game();
    //deserialize_game_and_start();
}

fn start_game() {
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

/// Load a previously saved game state
/// to be able to use the debugger.
fn deserialize_game_and_start() {
    run_loop(deserialize_game(), SystemTime::now(), true);
}

/*
Move (n, s, e w) | Cost: 10% of halite available at
turn origin cell is deducted from ship’s current halite.

Move (o) | Collect: 25% of halite available in cell,
rounded up to the nearest whole number.
*/