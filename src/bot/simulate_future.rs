use hlt::game::Game;
use simulator::memory::Memory;
use simulator::simulator::Simulator;
use bot::simulating_bot::SimulatingBot;
use std::time::SystemTime;

pub fn run(mut hlt_game: Game, start_time: SystemTime) {
    first_turn(&mut hlt_game);

    let mut memory = Memory::new();

    loop {
        if hlt_game.turn_number == 10 {
            first_turn(&mut hlt_game);
        }
        
        hlt_game.update_frame();
        let mut commands = Vec::new();

        let mut simulator = Simulator::new(&hlt_game, &mut memory);
        let my_ships = &hlt_game.me().ship_ids;

        for ship_id in my_ships {
            // Borrowing in loops is a nightmare.
            // Borrow &mut multiple times.
            unsafe {
                let sim: *mut Simulator = &mut simulator;
                let mut bot: SimulatingBot = SimulatingBot::new(
                    ship_id.clone(), &mut *sim);
                commands.push(bot.calculate_command());
            }
        }
        /*let ship = hlt_game.id_to_ship(ship_id);
        log(&format!(
            "Real: ship: {}, map: {}, pos: {} {}", ship.halite,
            hlt_game.game_map.at_position(&ship.position).halite,
            ship.position.x, ship.position.y)); */
        
        if hlt_game.turn_number >= hlt_game.constants.max_turns {
            let end_time = SystemTime::now();
            hlt_game.log.borrow_mut().log(&format!("Game took {:?}", end_time.duration_since(start_time)));
        }
        
        Game::end_turn(&commands);
    }
}

/// spawn ship
fn first_turn(game: &mut Game) {
    game.update_frame();
    let mut commands = Vec::new();
    commands.push(game.players[game.my_id.0].shipyard.spawn());
    Game::end_turn(&commands);
}