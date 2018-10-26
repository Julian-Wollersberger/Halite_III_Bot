use hlt::position::Position;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ComplexAction {
    Undefined, // Action must be calculated.
    //Dropoff, // Maybe the dropoff should play traffic control?
    Navigate(Position), // Only move
    NavigateCollect(Position), // Collect on the way
    //Attack,  // Don't fear collision

    //TODO Dodge(original Position), // Dodge another ship
    //DodgeCollect(original Position)
}
