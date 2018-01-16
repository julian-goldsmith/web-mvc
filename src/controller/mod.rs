use std::boxed::Box;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use stdweb::web::event::{IEvent, ConcreteEvent};
use stdweb::unstable::Void;

pub mod active_controller;
pub mod completed_controller;
pub mod root_controller;

pub use active_controller::ActiveController;
pub use completed_controller::CompletedController;
pub use root_controller::RootController;

pub trait Controller<State> {
    // FIXME: return values
    // navigate sets up event listeners
    fn navigate<'a>(&self, state: &'a mut State);
    
    // leave tears down event listeners
    fn leave<'a>(&self, state: &'a mut State);
}

pub type ControllerRef<State> = Rc<RefCell<Box<Controller<State>>>>;
