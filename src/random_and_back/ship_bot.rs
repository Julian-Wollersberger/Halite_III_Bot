extern crate rand;

use std::cell::RefCell;
use std::rc::Rc;
use hlt::ship::Ship;
use hlt::command::Command;
use hlt::log::Log;
use hlt::ShipId;
use hlt::direction::Direction;
use rand::Rng;
use hlt::game::Game;
use random_and_back::extended_map::ExtendedMap;
use random_and_back::complex_action::ComplexAction;
use hlt::position::Position;
use hlt::map_cell::Structure;
use core::mem;

/* This is a more intelligent ship.
 * It plans a few turns. */
pub struct ShipBot {
    pub ship_id: ShipId,
    logger: Rc<RefCell<Log>>,
    movement_blocked: u32,

    // Action for next turn. It may be set anywhere in
    // the logic chain.
    // If not set, the current action will continue.
    next_action: Option<ComplexAction>,
}

/* To prevent recursive endless loops,
 * Functions must only call functions that are
 * further down in this file. */
impl ShipBot {

    pub fn new(ship_id: &ShipId, logger: Rc<RefCell<Log>>) -> ShipBot {
        ShipBot {
            ship_id: ship_id.clone(),
            logger,
            movement_blocked: 0,
            //current_action: ComplexAction::still(),
            next_action: Some(ComplexAction::Undefined),
        }
    }

    /// Processes the AI to come up with a Command.
    /// Returns an Error if the ship doesn't exist anymore.
    pub fn next_turn(
        &mut self, game: &Game, ex_map: &mut ExtendedMap
    ) -> Result<Command, String> {

        // The next_action from previous turn
        // becomes the new current_action.
        let action_option = mem::replace(
            &mut self.next_action, None);
        let current_action = match action_option {
            Some(action) => action,
            None => {
                self.logger.borrow_mut().log("No action defined in previous turn!");
                ComplexAction::Undefined
            },
        };

        // First, find out if the ship still exists.
        let hlt_ship: &Ship;
        match game.ships.get(&self.ship_id) {
            Some(ship) => hlt_ship = ship,
            None =>
                return Result::Err(format!("The ship {} doesn't exist anymore!", &self.ship_id.0))
        }

        // Decide based on current action
        let direction = match current_action {
            ComplexAction::Navigate(destination) => {
                self.move_in_direction(&destination, hlt_ship, ex_map, game)
            },
            ComplexAction::NavigateCollect(destination) => {
                self.navigate_or_collect(&destination, hlt_ship, ex_map, game)
            }

            ComplexAction::Undefined => {
                self.decide_action(hlt_ship, ex_map, game)
            }
        };
        self.logger.borrow_mut().log(&format!(
            "Current Action: {:?}, next: {:?}, direction: {:?}",
            current_action, self.next_action, direction)[..]);

        // If no-one did set next_action, the current_action
        // should continue next turn.
        if self.next_action.is_none() {
            self.next_action = Some(current_action);
        }

        let command = hlt_ship.move_ship(direction);
        return Result::Ok(command);
    }


    fn decide_action(
        &mut self, ship: &Ship, ex_map: &mut ExtendedMap, game: &Game
    ) -> Direction {
        // If ship is at dropoff, navigate somewhere else
        if ex_map.game_map.at_position(&ship.position).structure != Structure::None {
            return self.navigate_random(ship, ex_map, game)

        // If full, go home.
        } else if ShipBot::is_full(ship, 0.80) {
            self.navigate_to_dropoff(ship, ex_map, game)

        // Default: navigate_collect to random location.
        } else {
            return self.navigate_random_collect(ship, ex_map, game)
        }
    }

    /// Moves farther away
    fn navigate_random(
        &mut self, ship: &Ship, ex_map: &mut ExtendedMap, game: &Game
    )-> Direction {
        const MAX_STEPS: i32 = 12;
        const MIN_STEPS: i32 = 8;

        let random = ShipBot::random_position_near(
            &ship.position, MIN_STEPS, MAX_STEPS);
        self.next_action = Some(ComplexAction::Navigate(random));
        self.navigate_or_collect(&random, ship, ex_map, game)
    }

    fn navigate_random_collect(
        &mut self, ship: &Ship, ex_map: &mut ExtendedMap, game: &Game
    )-> Direction {
        const MAX_STEPS: i32 = 8;
        const MIN_STEPS: i32 = 4;

        let random = ShipBot::random_position_near(
            &ship.position, MIN_STEPS, MAX_STEPS);
        self.next_action = Some(ComplexAction::NavigateCollect(random));
        self.move_in_direction(&random, ship, ex_map, game)
    }

    /// if ship is full, move to base
    /// if cell is empty, move further
    /// else collect.
    fn navigate_or_collect(
        &mut self, destination: &Position,
        ship: &Ship, ex_map: &mut ExtendedMap, game: &Game,
    )-> Direction {

        let cell_almost_empty = ex_map.game_map.at_entity(ship)
            .halite < game.constants.max_halite / 50;

        if ShipBot::is_full(ship, 0.95) {
            return self.navigate_to_dropoff(ship, ex_map, game);
        } else if cell_almost_empty {
            return self.move_in_direction(destination, ship, ex_map, game);
        } else {
            return Direction::Still;
        };
    }

    /// A ship is considered full if it has more than
    /// FULL_RATIO halite stored.
    fn is_full(ship: &Ship, factor: f64) -> bool {
        return ship.halite as f64 >= factor * ship.max_halite() as f64;
    }

    /// currently only shipyard.
    fn navigate_to_dropoff(
        &mut self, ship: &Ship, ex_map: &mut ExtendedMap, game: &Game
    ) -> Direction {

        let me = &game.players[game.my_id.0];
        let dropoff_pos = game.game_map.at_entity(&me.shipyard).position;

        self.next_action = Some(ComplexAction::NavigateCollect(dropoff_pos));
        return self.move_in_direction(&dropoff_pos, ship, ex_map, game)
    }

    fn move_in_direction(
        &mut self, destination: &Position,
        ship: &Ship, ex_map: &mut ExtendedMap, game: &Game
    ) -> Direction {

        // if arrived, decide what to do next.
        if destination.x == ship.position.x
            && destination.y == ship.position.y
        {
            self.next_action = Some(ComplexAction::Undefined);
            return Direction::Still
        }

        // non't move if no fuel
        if game.game_map.at_entity(ship)
            .halite /10 > ship.halite
        {
            return Direction::Still;
        }

        //Chose direction that brings the ship closer.
        let possible_dir = ex_map.game_map
            .get_unsafe_moves(&ship.position, destination);
        /*let move_dir = if possible_dir.len() == 0 {
                Direction::Still
            } else {
                possible_dir[rand::thread_rng().gen_range(0, possible_dir.len())]
            };*/
        let move_dir= match possible_dir.len() {
            0 => Direction::Still,
            1 => possible_dir[0],
            2 => {
                // choose the direction with more halite.
                if ex_map.game_map.at_position(&ship.position
                        .directional_offset(possible_dir[0])).halite
                    >= ex_map.game_map.at_position(&ship.position
                        .directional_offset(possible_dir[1])).halite
                {
                    possible_dir[0]
                } else { possible_dir[1] }
            },
            _ => {
                self.logger.borrow_mut().log("More that 2 directions!");
                possible_dir[0]
            }
        };

        if ex_map.can_move_safely_then_reserve(
            &ship.position.directional_offset(move_dir))
        {
            self.movement_blocked = 0;
            return move_dir;


        // TODO Proper dodge
        } else {
            self.movement_blocked += 1;

            if self.movement_blocked >= 7 {
                self.movement_blocked = 0;
                self.next_action = Some(ComplexAction::Navigate(
                    ShipBot::random_position_near(&ship.position, 4, 8)));
            }

            return Direction::Still;
        }
    }

    /// Position near the ship that is Distance movements away.
    /// Should give a distribution like two dice.
    fn random_position_near(pos: &Position, min_steps: i32, max_steps: i32) -> Position {
        Position {
            x: pos.x + ShipBot::pos_neg_range(min_steps/2, max_steps/2),
            y: pos.y + ShipBot::pos_neg_range(min_steps/2,max_steps/2),
        }
    }

    /// generates a number in the specified range,
    /// but it may also be negative.
    /// => range(-min, -max) or range(min, max)
    fn pos_neg_range(min: i32, max: i32) -> i32 {
        let mut rng = rand::thread_rng();
        return rng.gen_range(min,max)
            * if rng.gen_bool(0.5) { 1 } else { -1 } // +1 or -1
    }
}



