use std::any::Any;

use controller::{Controller, ControllerRef};
use model::State;

pub struct ActiveController {
}

impl Controller for ActiveController {
    fn as_any_mut(&mut self) -> &mut Any {
        self
    }

    fn navigate(&mut self, controller_ref: &ControllerRef) {
        js! {
            console.log("active navigated");
        };
    }

    fn leave(&mut self) {
        js! {
            console.log("active left");
        };
    }
}
