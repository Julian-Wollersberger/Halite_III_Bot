use hlt::ShipId;
use hlt::direction::Direction;

/// The equivalent of a Command, but for the game.
pub enum Action {
    MoveShip(ShipId, Direction),
    //CreateDropoff,
    None,
}