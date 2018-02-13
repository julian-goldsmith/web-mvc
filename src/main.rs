#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod controller;
mod model;

use controller::*;

use stdweb::web::{
    IEventTarget,
    document,
    window
};

use stdweb::web::event::HashChangeEvent;

fn navigate(controller: &ControllerRef) {
    // pick controller from route
    let hash = document().location().unwrap().hash().unwrap();
    let controller_new = match hash.as_str() {
        //"#/active" => Box::new(ActiveController { }),
        //"#/completed" => Box::new(CompletedController { }),
        _ => Box::new(RootController::new()),
    };

    let mut controller_old = controller.replace(controller_new);

    controller_old.leave();
    drop(controller_old);

    let mut controller_borrow = controller.borrow_mut();
    controller_borrow.navigate(controller);
}

fn main() {
    stdweb::initialize();

    let controller: ControllerRef = ControllerRef::new(Box::new(RootController::new()));

    window().add_event_listener({
        let controller = controller.clone();

        move |_: HashChangeEvent| {
            navigate(&controller);
        }
    });

    navigate(&controller);
    stdweb::event_loop();
}
