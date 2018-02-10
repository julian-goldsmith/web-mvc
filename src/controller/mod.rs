use std::boxed::Box;
use std::cell::{Ref,RefCell};
use std::ops::Deref;
use std::rc::Rc;

//pub mod active_controller;
//pub mod completed_controller;
pub mod root_controller;

//pub use active_controller::ActiveController;
//pub use completed_controller::CompletedController;
pub use root_controller::RootController;

pub trait Controller<State> {
    // FIXME: return values
    // navigate sets up event listeners
    fn navigate<'a>(&mut self, state: &'a State, controller_ref: &ControllerRef<State>);
    
    // leave tears down event listeners
    fn leave<'a>(&mut self, state: &'a State);
}

#[derive(Clone)]
pub struct ControllerRef<State> {
    inner: Rc<RefCell<Box<Controller<State>>>>,
}

impl<State> Deref for ControllerRef<State> {
    type Target = Rc<RefCell<Box<Controller<State>>>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<State> ControllerRef<State> {
    pub fn new(controller: Box<Controller<State>>) -> ControllerRef<State> {
        ControllerRef {
            inner: Rc::new(RefCell::new(controller))
        }
    }
}
