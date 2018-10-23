use std::cell::RefCell;
use std::rc::Rc;
use hlt::ship::Ship;
use hlt::command::Command;
use hlt::log::Log;
use hlt::ShipId;
use std::collections::HashMap;

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
    pub fn next_frame(&mut self, ships: &HashMap<ShipId, Ship>) -> Result<Command, ()> {
        // First, find out if the ship still exists.
        let hlt_ship: &Ship;
        match ships.get(&self.ship_id) {
            Some(ship) => hlt_ship = ship,
            None =>
                return Result::Err(())
        }

        // if queue empty
        if self.movement_queue.len() <= 0 {
            self.calculate_ai(hlt_ship);
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

    fn calculate_ai(&mut self, ship: &Ship) {
        self.movement_queue.push(ship.stay_still());
        self.movement_queue.push(ship.stay_still());
        self.movement_queue.push(ship.stay_still());
    }
}



