use stdweb::unstable::TryInto;

use stdweb::web::{
    IEventTarget,
    document,
};

use stdweb::web::event::{
    KeypressEvent,
};

use stdweb::web::html_element::InputElement;

use controller::Controller;
use model::State;

pub struct RootController {
    title_entry: InputElement,
}

impl Controller<State> for RootController {
    fn navigate<'a>(&mut self, state: &'a mut State) {
        self.title_entry.add_event_listener(|_: KeypressEvent| {
            js! {
                console.log("key press");
            }
        });
    }

    fn leave<'a>(&mut self, state: &'a mut State) {
    }
}

impl RootController {
    pub fn new() -> Self {
        let title_entry: InputElement = document().query_selector(".new-todo").unwrap().try_into().unwrap();
        RootController { title_entry }
    }
}
