#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod controller;

use controller::Controller;

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

#[derive(Clone, Serialize, Deserialize)]
struct Todo {
    title: String,
    completed: bool
}

#[derive(Serialize, Deserialize)]
struct State {
    todo_list: Vec< Todo >
}

impl State {
    fn new() -> Self {
        State {
            todo_list: Vec::new()
        }
    }
}

type StateRef = Rc<RefCell<State>>;
type ControllerRef = Rc<RefCell<Box<Controller<State>>>>;

struct RootController {
}

impl Controller<State> for RootController {
    fn navigate<'a>(&self, state: &'a mut State) {
        js! {
            console.log("root navigated");
        };
    }

    fn leave<'a>(&self, state: &'a mut State) {
        js! {
            console.log("root left");
        };
    }
}

struct ActiveController {
}

impl Controller<State> for ActiveController {
    fn navigate<'a>(&self, state: &'a mut State) {
        js! {
            console.log("active navigated");
        };
    }

    fn leave<'a>(&self, state: &'a mut State) {
        js! {
            console.log("active left");
        };
    }
}

struct CompletedController {
}

impl Controller<State> for CompletedController {
    fn navigate<'a>(&self, state: &'a mut State) {
        js! {
            console.log("completed navigated");
        };
    }

    fn leave<'a>(&self, state: &'a mut State) {
        js! {
            console.log("completed left");
        };
    }
}


fn update_dom(state: &StateRef, controller: &ControllerRef) {
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

    let mut controller: ControllerRef = Rc::new(RefCell::new(Box::new(RootController { })));

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
