use hlt::command::Command;
use hlt::direction::Direction;
use hlt::entity::Entity;
use hlt::input::Input;
use hlt::PlayerId;
use hlt::position::Position;
use hlt::ShipId;

#[derive(Clone, Serialize, Deserialize)]
pub struct Ship {
    pub owner: PlayerId,
    pub id: ShipId,
    pub position: Position,
    pub halite: usize,
    max_halite: usize,
}

impl Ship {
    pub fn is_full(&self) -> bool {
        self.halite >= self.max_halite
    }

    pub fn max_halite(&self) -> usize {
        self.max_halite
    }

    pub fn make_dropoff(&self) -> Command {
        Command::transform_ship_into_dropoff_site(self.id)
    }

    pub fn move_ship(&self, direction: Direction) -> Command {
        Command::move_ship(self.id, direction)
    }

    pub fn stay_still(&self) -> Command {
        Command::move_ship(self.id, Direction::Still)
    }

    pub fn generate(input: &mut Input, player_id: PlayerId, max_halite: usize) -> Ship {
        // Why is this a variable?
        assert_eq!(max_halite, 1000);

        input.read_and_parse_line();
        let id = ShipId(input.next_usize());
        let x = input.next_i32();
        let y = input.next_i32();
        let halite = input.next_usize();

        Ship { owner: player_id, id, position: Position { x, y }, halite, max_halite }
    }
}

impl Entity for Ship {
    fn owner(&self) -> PlayerId {
        self.owner
    }

    fn position(&self) -> Position {
        self.position
    }
}


#[cfg(test)]
pub mod test {
    use hlt::ship::Ship;
    use hlt::PlayerId;
    use hlt::ShipId;
    use hlt::position::Position;

    pub fn sample_ship(pos: Position) -> Ship {
        static mut SHIP_ID_COUNTER: usize = 0;

        unsafe {
            let id = SHIP_ID_COUNTER;
            SHIP_ID_COUNTER += 1;

            Ship {
                owner: PlayerId(1),
                id: ShipId(id),
                position: pos,
                halite: 0,
                max_halite: 1000,
            }
        }
    }

    #[test]
    fn sample_ship_test() {
        let first = sample_ship(Position{x:0,y:0}).id.0;
        assert_eq!(sample_ship(Position{x:0,y:0}).id.0, first + 1);
        assert_eq!(sample_ship(Position{x:0,y:0}).id.0, first + 2);
        assert_eq!(sample_ship(Position{x:0,y:0}).id.0, first + 3);
    }
}