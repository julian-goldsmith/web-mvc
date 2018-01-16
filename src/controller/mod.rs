use std::boxed::Box;
use std::cell::RefCell;
use std::rc::Rc;

pub mod active_controller;
pub mod completed_controller;
pub mod root_controller;

pub use active_controller::ActiveController;
pub use completed_controller::CompletedController;
pub use root_controller::RootController;

pub trait Controller<State> {
    // FIXME: return values
    // navigate sets up event listeners
    fn navigate<'a>(&mut self, state: &'a mut State);
    
    // leave tears down event listeners
    fn leave<'a>(&mut self, state: &'a mut State);
}

pub type ControllerRef<State> = Rc<RefCell<Box<Controller<State>>>>;
