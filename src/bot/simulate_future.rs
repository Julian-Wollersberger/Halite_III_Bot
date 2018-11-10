use hlt::game::Game;
use simulator::memory::Memory;
use simulator::simulator::Simulator;

pub fn run(mut hlt_game: Game) {
    first_turn(&mut hlt_game);

    let mut memory = Memory {};

    loop {
        hlt_game.update_frame();
        let mut commands = Vec::new();

        let simulator = Simulator::new(&hlt_game, &mut memory);



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