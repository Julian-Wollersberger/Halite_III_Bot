use hlt::log::Log;
use std::cell::RefCell;
use std::rc::Rc;

/// A global reference to the game's LOGGER.
pub fn log(m: &str) {
    unsafe {
        let loggggggger = LOGGER.clone();
        match loggggggger {
            Some(l) => l.borrow_mut().log(&m),
            None => {}
        }
    }
}

pub fn set_logger(logger: Rc<RefCell<Log>>) {
    unsafe {
        LOGGER = Some(logger);
    }
}

static mut LOGGER: Option<Rc<RefCell<Log>>> = None;


