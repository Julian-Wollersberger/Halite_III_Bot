use simulator::memory::Memory;
use std::rc::Rc;
use std::cell::RefCell;
use hlt::log::Log;
use simulator::simulator::Simulator;
use hlt::ShipId;

pub struct SimulatingBot<'turn > {
    simulator: &'turn Simulator<'turn>,
    memory: &'turn Memory,
    logger: Rc<RefCell<Log>>,

    id: ShipId,
}

impl<'turn> SimulatingBot<'turn> {

    pub fn new<'t>(
        simulator: &'t Simulator,
        memory: &'t Memory,
        logger: Rc<RefCell<Log>>,
        id: ShipId,
    ) -> SimulatingBot<'t> {
        SimulatingBot { simulator, memory, logger, id, }
    }


}