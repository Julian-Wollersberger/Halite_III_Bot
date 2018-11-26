use std::collections::HashMap;
use hlt::ShipId;
use hlt::ship::Ship;
use hlt::position::Position;
use simulator::Halite;

/// A diff to the game state.
/// Only the halite in a few cells is collected.
/// Copying the entire halite_map would be expensive.
#[derive(Clone)]
pub struct StateDifference {
    ships: HashMap<ShipId, Ship>,
    //Fast access for ship_at()
    ship_pos: HashMap<Position, ShipId>,
    //Diff to real map
    halite: HashMap<Position, Halite>,
}

impl StateDifference {
    
    pub fn new() -> StateDifference {
        // TODO Cache the HashMaps.
        StateDifference {
            ships: HashMap::new(),
            ship_pos: HashMap::new(),
            halite: HashMap::new(),
        }
    }
    
    pub fn ship(&self, id: ShipId) -> Option<&Ship> {
        self.ships.get(&id)
    }
    /// old is needed to update the position correctly.
    pub fn set_ship(&mut self, new: Ship, old: Option<&Ship>) {
        if let Some(old_ship) = old {
            assert_eq!(old_ship.id.0, new.id.0);
            self.ship_pos.remove(&old_ship.position);
        }
        self.ship_pos.insert(new.position, new.id);
        self.ships.insert(new.id, new);
    }
    
    pub fn ship_at(&self, pos: Position) -> Option<ShipId> {
        self.ship_pos.get(&pos).cloned()
    }
    
    pub fn halite(&self, pos: &Position) -> Option<Halite> {
        self.halite.get(pos).cloned()
    }
    pub fn set_halite(&mut self, pos: Position, halite: Halite) {
        self.halite.insert(pos, halite);
    }
    
    pub fn clear(&mut self) {
        self.ships.clear();
        self.ship_pos.clear();
        self.halite.clear();
    }
    /// Overwrite existing entries.
    pub fn extend(&mut self, with: StateDifference) {
        self.ships.extend(with.ships);
        self.ship_pos.extend(with.ship_pos);
        self.halite.extend(with.halite);
    }
}

#[cfg(test)]
mod tests {
    use simulator::state_difference::StateDifference;
    use hlt::ship::test::sample_ship;
    use hlt::position::Position;
    
    #[test]
    fn ships_set_get () {
        let mut diff = StateDifference::new();
        let pos_1 = Position{x:4,y:8};
        let ship = sample_ship(pos_1);
        diff.set_ship(ship.clone(), None);
    }
}