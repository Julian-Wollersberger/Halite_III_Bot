use hlt::game::Game;
use hlt::command::Command;

use ship_bot::ShipBot;

pub fn run(mut game: Game) {
    let mut ship_bot = spawn_initial_ship(&mut game);

    loop {
        game.update_frame();

        let command_queue = Vec::new();
        ship_bot.next_frame(&game.ships);

        Game::end_turn(&command_queue);
    }
}

/* First two turns */
fn spawn_initial_ship(game: &mut Game) -> ShipBot {
    game.update_frame();
    let mut command_queue: Vec<Command> = Vec::new();
    {
        let me = &game.players[game.my_id.0];
        command_queue.push(me.shipyard.spawn());
    } // End mutable borrow of game

    Game::end_turn(&command_queue);
    game.update_frame();

    let me = &game.players[game.my_id.0];
    let ship = &game.ships[&me.ship_ids[0]];

    Game::end_turn(&Vec::new());
    return ShipBot::generate(&ship, game.log.clone());
}

