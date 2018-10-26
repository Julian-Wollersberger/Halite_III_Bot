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
use extended_map::ExtendedMap;
use complex_action::ComplexAction;
use hlt::position::Position;
use hlt::map_cell::Structure;

/* This is a more intelligent ship.
 * It plans a few turns. */
pub struct ShipBot {
    pub ship_id: ShipId,
    logger: Rc<RefCell<Log>>,
    //not_moved: u32,

    //complex_com_queue: Vec<ComplexCommand>,
    //current_action: ComplexAction,
    next_action: ComplexAction,
}

/* To prevent recursive endless loops,
 * Functions must only call functions that are
 * further down in this file. */
impl ShipBot {

    pub fn new(ship_id: &ShipId, logger: Rc<RefCell<Log>>) -> ShipBot {
        ShipBot {
            ship_id: ship_id.clone(),
            logger,
            //current_action: ComplexAction::still(),
            next_action: ComplexAction::Undefined,
        }
    }

    /// Processes the AI to come up with a Command.
    /// Returns an Error if the ship doesn't exist anymore.
    pub fn next_turn(
        &mut self, game: &Game, ex_map: &mut ExtendedMap
    ) -> Result<Command, String> {

        let current_action = self.next_action;

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
                self.collect_or_move(&destination, hlt_ship, ex_map, game)
            }

            ComplexAction::Undefined => {
                self.decide_action(hlt_ship, ex_map, game)
            }
        };


        // Pop one action per round.
        // Try to move ship, but stay still if a collision would occur.
        /*    Some(direction) => {
                if ex_map.can_move_safely_then_reserve(&hlt_ship.position.directional_offset(direction)) {
                    hlt_ship.move_ship(direction)
                } else {
                    if direction != Direction::Still {
                        retry = Some(direction);
                        self.not_moved += 1;
                    }
                    hlt_ship.stay_still()
                }
            },
            None => { // Fail-safe: Stay still.
                self.logger.borrow_mut().log("ShipBot: The AI didn't add Actions!");
                hlt_ship.stay_still() */
        // If not moved for to long, try some other random movement

        return Result::Ok(command);
    }

    fn decide_action(
        &mut self, ship: &Ship, ex_map: &mut ExtendedMap, game: &Game
    ) -> Direction {
        const FULL_RATIO: f64 = 0.9;

        // If ship is at dropoff, navigate somewhere else
        if ex_map.game_map.at_position(&ship.position).structure != Structure::None {
            return self.navigate_to_random_position(ship, ex_map, game)

        // If full, go home.
        } else if ship.halite as f64 >= FULL_RATIO * ship.max_halite() as f64 {
            let dropoff_pos = self.find_dropoff(ship, game);
            self.next_action = ComplexAction::Navigate(dropoff_pos);
            self.move_in_direction(&dropoff_pos, ship, ex_map, game)

        // Default: navigate_collect to home.
        } else {
            let dropoff_pos = self.find_dropoff(ship, game);
            self.next_action = ComplexAction::NavigateCollect(dropoff_pos);
            self.collect_or_move(&dropoff_pos, ship, ex_map, game)

        }
    }

    fn collect_or_move(
        &mut self, destination: &Position,
        ship: &Ship, ex_map: &mut ExtendedMap, game: &Game,
    )-> Direction {
        // TODO fn Deside movement


        Direction::Still
    }

    fn navigate_to_random_position(
        &mut self, ship: &Ship, ex_map: &mut ExtendedMap, game: &Game
    )-> Direction {
        const MAX_STEPS: i32 = 13;
        const MIN_STEPS: i32 = 11;

        let random = ShipBot::random_position_near(ship, MIN_STEPS, MAX_STEPS);
        self.next_action = ComplexAction::Navigate(random);
        self.move_in_direction(&random, ship, ex_map, game)
    }

    /// Position near the ship that is Distance movements away.
    /// Should give a distribution like two dice.
    fn random_position_near(ship: &Ship, min_distance: i32, max_distance: i32) -> Position {
        Position {
            x: rand::thread_rng().gen_range(min_distance/2,max_distance/2),
            y: rand::thread_rng().gen_range(min_distance/2,max_distance/2),
        }
    }

    /// Is the shipyard a dropoff?
    fn find_dropoff(
        &mut self, ship: &Ship, game: &Game
    ) -> Position {

        // get dropoff and use it as destination.
        if let Some(dropoff) = game.dropoffs.values().next() {
            return dropoff.position.clone();

        } else {
            self.logger.borrow_mut().log("No Dropoff :'(");
            return ship.position.clone();
        }
    }

    fn move_in_direction(
        &mut self, destination: &Position,
        ship: &Ship, ex_map: &mut ExtendedMap, game: &Game
    ) -> Direction {

        // if arrived, move back and collect
        if destination.x == ship.position.x
            && destination.y == ship.position.y
        {
            self.next_action = ComplexAction::Undefined;
            return Direction::Still
        }

        //TODO Don't naive_navigate, but chose random direction.
        let move_dir =ex_map.game_map.naive_navigate(&ship, destination);
        if ex_map.can_move_safely_then_reserve(
            &ship.position.directional_offset(move_dir))
        {
            return move_dir;
        } else {
            // fixme Deadlock -> dodge
            return Direction::Still;
        }
    }
}



