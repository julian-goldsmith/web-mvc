#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod controller;
mod model;

use controller::*;
use model::*;

use std::cell::RefCell;
use std::rc::Rc;

use stdweb::web::{
    IEventTarget,
    document,
    window
};

use stdweb::web::event::HashChangeEvent;

fn navigate(state: &StateRef, controller: &ControllerRef<State>) {
    let mut state_borrow = state.borrow_mut();
    let mut controller_borrow = controller.borrow_mut();

    controller_borrow.leave(&mut state_borrow);

    // pick controller from route
    let hash = document().location().unwrap().hash();
    *controller_borrow = match hash.as_str() {
        //"#/active" => Box::new(ActiveController { }),
        //"#/completed" => Box::new(CompletedController { }),
        _ => Box::new(RootController::new()),
    };

    controller_borrow.navigate(&mut state_borrow);

    state_borrow.save();
}

fn main() {
    stdweb::initialize();

    let state = State::get_from_storage();

    let mut controller: ControllerRef<State> = Rc::new(RefCell::new(Box::new(RootController::new())));

    window().add_event_listener({
        let state = state.clone();
        let controller = controller.clone();

        move |_: HashChangeEvent| {
            navigate(&state, &controller);
        }
    });

    navigate(&state, &controller);
    stdweb::event_loop();
}
