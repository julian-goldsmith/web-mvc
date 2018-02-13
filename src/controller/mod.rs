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

pub trait Controller {
    // FIXME: return values
    // navigate sets up event listeners
    fn navigate(&mut self, controller_ref: &ControllerRef);
    
    // leave tears down event listeners
    fn leave(&mut self);
}

#[derive(Clone)]
pub struct ControllerRef {
    inner: Rc<RefCell<Box<Controller>>>,
}

impl Deref for ControllerRef {
    type Target = Rc<RefCell<Box<Controller>>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl ControllerRef {
    pub fn new(controller: Box<Controller>) -> ControllerRef {
        ControllerRef {
            inner: Rc::new(RefCell::new(controller))
        }
    }
}
