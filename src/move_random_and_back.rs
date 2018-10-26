use hlt::game::Game;
use hlt::command::Command;

use ship_bot::ShipBot;
use std::collections::HashMap;
use hlt::ShipId;
use extended_map::ExtendedMap;

pub fn run(mut game: Game) {
    // There may be stale/destroyed ships in this map.
    let mut bot_list: HashMap<ShipId, ShipBot> = HashMap::new();

    loop {
        game.update_frame();
        let mut command_queue = Vec::new();
        let mut extended_map = ExtendedMap::new(&game.game_map);

        maybe_spawn_ship(&mut extended_map, &game, &mut command_queue);
        process_ship_bots(&mut extended_map, &game, &mut command_queue, &mut bot_list);

        Game::end_turn(&command_queue);
    }
}

fn maybe_spawn_ship(
    extended_map: &mut ExtendedMap,
    game: &Game,
    command_queue: &mut Vec<Command>
) {
    const SPAWN_UNTIL_TURN: usize = 130;
    const MAX_SHIP_COUNT: usize = 15;

    let me = &game.players[game.my_id.0];
    let shipyard_cell = game.game_map.at_entity(&me.shipyard);

    if me.ship_ids.len() < MAX_SHIP_COUNT &&
        game.turn_number <= SPAWN_UNTIL_TURN &&
        me.halite >= game.constants.ship_cost &&
        extended_map.can_move_safely_then_reserve(&shipyard_cell.position)
    {
        command_queue.push(me.shipyard.spawn());
    }
}

fn process_ship_bots(
    extended_map: &mut ExtendedMap,
    game: &Game,
    command_queue: &mut Vec<Command>,
    bot_list: &mut HashMap<ShipId, ShipBot>
) {
    let me = &game.players[game.my_id.0];

    for ship_id in &me.ship_ids {
        // If no bot was created for this ship, add a new one.
        let ship_bot = bot_list.entry(ship_id.clone())
            .or_insert(ShipBot::new(ship_id, game.log.clone()));

        // Process the ship bots
        match ship_bot.next_turn(&game, extended_map) {
            Ok(command) => command_queue.push(command),
            Err(message) => print!("{}", message)
        };
    }
}
