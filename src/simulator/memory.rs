use hlt::ShipId;
use std::collections::HashMap;
use hlt::direction::Direction;
use std::cell::RefCell;
use hlt::ship::Ship;
use hlt::position::Position;
use simulator::logger::log;

/// Persistent across turns.
/// The hlt_game replaces all its objects,
/// so none of them can be borrowed until the next turn.
pub struct Memory {
    ship_path: RefCell<HashMap<ShipId, Vec<Direction>>>,
    /// Overwrites at a certain turn
    cell_overwrites: RefCell<HashMap<u32, HashMap<Position, u16>>>,
    ship_overwrites: RefCell<HashMap<u32, HashMap<ShipId, Ship>>>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            ship_path: RefCell::new(HashMap::new()),
            cell_overwrites: RefCell::new(HashMap::new()),
            ship_overwrites: RefCell::new(HashMap::new()),
        }
    }

    pub fn store_path(&self, id: ShipId, dir: Vec<Direction>) {
        self.ship_path.borrow_mut().insert(id, dir);
    }
    pub fn ship_path(&self, id: &ShipId) -> Vec<Direction> {
        match self.ship_path.borrow_mut().remove(id) {
            Some(dir) => dir,
            None => Vec::new(),
        }
    }
    
    /// Add the cell_overwrites to the existing ones.
    pub fn store_cell_ow(&self, turn_number: u32, cells: HashMap<Position, u16>) {
        self.cell_overwrites.borrow_mut()
            .entry(turn_number)
            .or_insert_with(|| HashMap::new())
            .extend(cells);
        
        //log(&format!("Storing {} cells in total on turn {}",
        //    self.cell_ow_on(turn_number).len(), turn_number));
    }
    pub fn cell_ow_on(&self, turn_number: u32) -> HashMap<Position, u16> {
        match self.cell_overwrites.borrow_mut().get(&turn_number) {
            Some(map) => {
                //log(&format!("Restoring {} cells on turn {}.", map.len(), turn_number));
                map.clone()
            },
            None => HashMap::new(),
        }
    }
    
    /// Add the ship_overwrites to the existing ones.
    pub fn store_ship_ow(&self, turn_number: u32, ships: HashMap<ShipId, Ship>) {
        self.ship_overwrites.borrow_mut()
            .entry(turn_number)
            .or_insert_with(|| HashMap::new())
            .extend(ships);
    
        //log(&format!("Storing {} ships in total on turn {}",
        //    self.ship_ow_on(turn_number).len(), turn_number));
    }
    pub fn ship_ow_on(&self, turn_number: u32) -> HashMap<ShipId, Ship> {
        match self.ship_overwrites.borrow_mut().get(&turn_number) {
            Some(map) => {
                //log(&format!("Restoring {} ships on turn {}.", map.len(), turn_number));
                map.clone()
            },
            None => {
                //log(&format!("New ship HashMap on turn {}.", turn_number));
                HashMap::new()
            },
        }
    }
    
    pub fn clean_up(&mut self) {
        // TODO Remove destroyed ships and past turns.
    }
}