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

use stdweb::web::event::{
    IEvent,
    ConcreteEvent,
    HashChangeEvent
};

fn update_dom(state: &StateRef, controller: &ControllerRef<State>) {
    let mut state_borrow = state.borrow_mut();
    let mut controller_borrow = controller.borrow_mut();

    controller_borrow.leave(&mut state_borrow);

    // pick controller from route
    let hash = document().location().unwrap().hash();
    *controller_borrow = match hash.as_str() {
        "#/active" => Box::new(ActiveController { }),
        "#/completed" => Box::new(CompletedController { }),
        _ => Box::new(RootController { }),
    };

    controller_borrow.navigate(&mut state_borrow);

    // Save the state into local storage.
    let state_json = serde_json::to_string(&*state_borrow).unwrap();
    window().local_storage().insert("state", state_json.as_str());
}

fn main() {
    stdweb::initialize();

    let state = Rc::new(RefCell::new(
        window().
        local_storage().
        get("state").
        and_then(|state_json| {
            serde_json::from_str(state_json.as_str()).ok()
        }).
        unwrap_or_else(State::new)));

    let mut controller: ControllerRef<State> = Rc::new(RefCell::new(Box::new(RootController { })));

    window().add_event_listener({
        let state = state.clone();
        let controller = controller.clone();

        move |_: HashChangeEvent| {
            update_dom(&state, &controller);
        }
    });

    update_dom(&state, &controller);
    stdweb::event_loop();
}
