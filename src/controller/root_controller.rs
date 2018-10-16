use std::any::Any;

use stdweb::unstable::TryInto;
use stdweb::web::html_element::InputElement;
use stdweb::web::{
    IEventTarget,
    IParentNode,
    document,
};
use stdweb::web::event::{
    IEvent,
    IKeyboardEvent,
    KeyPressEvent,
};

use controller::{Controller, ControllerRef};
use model::{State, Todo};

pub struct RootController {
    title_entry: InputElement,

    state: State,
}

impl Controller for RootController {
    fn as_any_mut(&mut self) -> &mut Any {
        self
    }

    fn navigate(&mut self, controller_ref: &ControllerRef) {
        self.title_entry.add_event_listener({
            let controller_ref = controller_ref.clone();
            move |event: KeyPressEvent|  {
                let mut controller = controller_ref.borrow_mut();
                let controller: &mut RootController = controller.as_any_mut().downcast_mut().unwrap();
                controller.key_press(event);
            }
        });
    }

    fn leave(&mut self) {
        self.state.save();
    }
}

impl RootController {
    pub fn new() -> Self {
        let title_entry: InputElement = document().query_selector(".new-todo").unwrap().unwrap().try_into().unwrap();
        RootController { title_entry, state: State::get_from_storage() }
    }

    fn key_press(&mut self, event: KeyPressEvent) {
        if event.key() == "Enter" {
            event.prevent_default();

            let title = self.title_entry.value().try_into().unwrap();

            self.state.todo_list.push(Todo { title, completed: false });

            js! {
                console.log(@{format!("{:?}", self.state.todo_list.clone())});
            }
        }
    }
}
