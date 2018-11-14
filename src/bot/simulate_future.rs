use hlt::game::Game;
use simulator::memory::Memory;
use simulator::simulator::Simulator;
use bot::simulating_bot::SimulatingBot;

pub fn run(mut hlt_game: Game) {
    first_turn(&mut hlt_game);

    let mut memory = Memory::new();

    loop {
        hlt_game.update_frame();
        let mut commands = Vec::new();

        let simulator = Simulator::new(&hlt_game, &mut memory);
        let my_ships = &hlt_game.me().ship_ids;

        for ship_id in my_ships {
            let mut bot = SimulatingBot::new(ship_id.clone(), &simulator, hlt_game.log.clone());
            commands.push(bot.calculate());
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