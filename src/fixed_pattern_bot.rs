use hlt::game::Game;
use hlt::command::Command;
use hlt::direction::Direction;


/* To practise the API, I'll send a ship around in a loop. */
pub fn run(mut game: Game) {
    let mut initial_turn = true;
    let mut programmed_commands: Vec<Command> = Vec::new();

    loop {
        game.update_frame();

        let mut command_queue: Vec<Command> = Vec::new();
        let me = &game.players[game.my_id.0];

        // Initial turn spawns a ship.
        if initial_turn {
            command_queue.push(me.shipyard.spawn());
            initial_turn = false;

        // Repeat the round
        } else if programmed_commands.len() <= 0 {
            let ship = &game.ships[&me.ship_ids[0]];

            programmed_commands.push(ship.move_ship(Direction::North));
            programmed_commands.push(ship.stay_still());
            programmed_commands.push(ship.move_ship(Direction::East));
            programmed_commands.push(ship.stay_still());
            programmed_commands.push(ship.move_ship(Direction::South));
            programmed_commands.push(ship.stay_still());
            programmed_commands.push(ship.move_ship(Direction::South));
            programmed_commands.push(ship.stay_still());
            programmed_commands.push(ship.move_ship(Direction::West));
            programmed_commands.push(ship.stay_still());
            programmed_commands.push(ship.move_ship(Direction::West));
            programmed_commands.push(ship.stay_still());
            programmed_commands.push(ship.move_ship(Direction::North));
            programmed_commands.push(ship.stay_still());
            programmed_commands.push(ship.move_ship(Direction::East));
            programmed_commands.push(ship.stay_still());
        }

        // Do one action per round.
        match programmed_commands.pop() {
            Some(com) => command_queue.push(com),
            None => game.log.borrow_mut().log("No programmed commands :(")
        }

        Game::end_turn(&command_queue);
    }
}