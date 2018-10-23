extern crate rand;

use std::cell::RefCell;
use std::rc::Rc;
use hlt::ship::Ship;
use hlt::command::Command;
use hlt::log::Log;
use hlt::ShipId;
use std::collections::HashMap;
use hlt::direction::Direction;
use rand::Rng;
use hlt::game_map::GameMap;

/* This is a more intelligent ship.
 * It has a command queue for the next few turns. */
pub struct ShipBot {
    pub ship_id: ShipId,
    movement_queue: Vec<Command>,
    logger: Rc<RefCell<Log>>,
}

impl ShipBot {

    pub fn generate(ship: &Ship, logger: Rc<RefCell<Log>>) -> ShipBot {
        ShipBot {
            ship_id: ship.id,
            movement_queue: Vec::new(),
            logger,
        }
    }

    /* Returns a queued action or
     * processes the AI to come up with actions.
     * Returns an Error if the ship doesn't exist anymore. */
    pub fn next_frame(&mut self, ships: &HashMap<ShipId, Ship>, game_map: &GameMap) -> Result<Command, ()> {
        // First, find out if the ship still exists.
        let hlt_ship: &Ship;
        match ships.get(&self.ship_id) {
            Some(ship) => hlt_ship = ship,
            None =>
                return Result::Err(())
        }

        // if queue empty
        if self.movement_queue.len() <= 0 {
            self.calculate_ai(hlt_ship, game_map);
        }

        // Pop one action per round.
        return Result::Ok( match self.movement_queue.pop() {
            Some(com) => com,
            None => { // Fail-safe: Stay still.
                self.logger.borrow_mut().log("ShipBot: The AI didn't add Actions!");
                hlt_ship.stay_still()
            }
        });
    }

    fn calculate_ai(&mut self, ship: &Ship, game_map: &GameMap) {
        const MAX_STEPS: i32 = 13;
        const MIN_STEPS: i32 = 11;
        // Make sure to only move in one quadrant.
        // Don't let random movement cancel itself out.

        let vertical_direction =
            if rand::thread_rng().gen_bool(0.5) {
                Direction::North
            } else { Direction::South };

        let horizontal_direction =
            if rand::thread_rng().gen_bool(0.5) {
                Direction::East
            } else { Direction::West };

        let num_steps = rand::thread_rng().gen_range(MIN_STEPS,MAX_STEPS);
        let mut directions: Vec<Direction> = Vec::new();

        for _ in 0..num_steps {
            // Either go horizontally or vertically.
            if rand::thread_rng().gen_bool(0.5) {
                directions.push(vertical_direction)
            } else {
                directions.push(horizontal_direction)
            };
        }
        // and now backwards and collect stuff.
        for direction in directions.iter() {
            self.movement_queue.push(ship.move_ship(
                direction.clone()));

            self.movement_queue.push(ship.stay_still());
            self.movement_queue.push(ship.stay_still());
            //self.movement_queue.push(ship.stay_still());
        }
        // First forwards.
        for direction in directions.iter().rev() {
            self.movement_queue.push(ship.move_ship(
                direction.invert_direction()));
            self.movement_queue.push(ship.stay_still());
        }
    }
}



