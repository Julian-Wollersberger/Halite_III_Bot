use hlt::position::Position;

#[derive(Clone, PartialEq, Eq)]
pub enum ComplexAction {
    Undefined, // Action must be calculated.
    //Dropoff, // Maybe the dropoff should play traffic control?
    Navigate(Position), // Only move
    NavigateCollect(Position), // Collect on the way
    //Attack,  // Don't fear collision
    //Dodge, // Dodge another ship
}
