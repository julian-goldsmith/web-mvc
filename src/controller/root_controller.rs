use stdweb::unstable::TryInto;

use stdweb::web::{
    IEventTarget,
    document,
};

use stdweb::web::event::{
    KeypressEvent,
};

use stdweb::web::html_element::InputElement;

use controller::{Controller, ControllerRef};
use model::{State, StateRef};

pub struct RootController {
    title_entry: InputElement,
}

impl Controller<StateRef> for RootController {
    fn navigate<'a>(&mut self, state: &'a StateRef, controller_ref: &ControllerRef<StateRef>) {
        self.title_entry.add_event_listener({
            let state = state.clone();
            move |event: KeypressEvent|  {
                RootController::key_press(&state, event);
            }
        });
    }

    fn leave<'a>(&mut self, state: &'a StateRef) {
    }
}

impl RootController {
    pub fn new() -> Self {
        let title_entry: InputElement = document().query_selector(".new-todo").unwrap().try_into().unwrap();
        RootController { title_entry }
    }

    fn key_press<'a>(state: &'a StateRef, event: KeypressEvent) {
        js! {
            console.log("key press" + @{&event});
        }
    }
}
