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
use model::{State, StateRef, Todo};

pub struct RootController {
    title_entry: InputElement,
}

impl Controller<StateRef> for RootController {
    fn navigate<'a>(&mut self, state: &'a StateRef, controller_ref: &ControllerRef<StateRef>) {
        self.title_entry.add_event_listener({
            let mut state = state.clone();
            let mut controller_ref = controller_ref.clone();
            move |event: KeyPressEvent|  {
                let mut state = state.borrow_mut();
                let mut controller = controller_ref.borrow_mut();
                controller.key_press(&mut state, event);
            }
        });
    }

    fn leave<'a>(&mut self, state: &'a StateRef) {
    }
}

impl RootController {
    pub fn new() -> Self {
        let title_entry: InputElement = document().query_selector(".new-todo").unwrap().unwrap().try_into().unwrap();
        RootController { title_entry }
    }

    fn key_press<'a>(&mut self, state: &'a mut State, event: KeyPressEvent) {
        if event.key() == "Enter" {
            event.prevent_default();

            let title_box: InputElement = document().query_selector(".new-todo").unwrap().unwrap().try_into().unwrap();
            let title = title_box.value().try_into().unwrap();

            state.todo_list.push(Todo { title, completed: false });

            js! {
                console.log(@{format!("{:?}", state.todo_list.clone())});
            }
        }
    }
}
