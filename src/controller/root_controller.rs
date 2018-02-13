use stdweb::unstable::TryInto;

use stdweb::web::{
    IEventTarget,
    document,
};

use stdweb::web::event::{
    IEvent,
    IKeyboardEvent,
    KeyPressEvent,
};

use stdweb::web::IParentNode;

use stdweb::web::html_element::InputElement;

use controller::{Controller, ControllerRef};
use model::{State, Todo};

pub struct RootController {
    title_entry: InputElement,

    state: State,
}

impl Controller for RootController {
    fn navigate<'a>(&mut self, controller_ref: &ControllerRef) {
        self.title_entry.add_event_listener({
            let mut controller_ref = controller_ref.clone();
            move |event: KeyPressEvent|  {
                let mut controller = controller_ref.borrow_mut();
                controller.key_press(event);
            }
        });
    }

    fn leave<'a>(&mut self) {
    }
}

impl RootController {
    pub fn new() -> Self {
        let title_entry: InputElement = document().query_selector(".new-todo").unwrap().unwrap().try_into().unwrap();
        RootController { title_entry, state: State::new() }
    }

    fn key_press<'a>(&mut self, event: KeyPressEvent) {
        if event.key() == "Enter" {
            event.prevent_default();

            let title = self.title_entry.value().try_into().unwrap();

            self.state.todo_list.push(Todo { title, completed: false });

            js! {
                console.log(@{format!("{:?}", state.todo_list.clone())});
            }
        }
    }
}
