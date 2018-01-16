use std::cell::RefCell;
use std::rc::Rc;

use serde_json;

use stdweb::web::window;

#[derive(Clone, Serialize, Deserialize)]
pub struct Todo {
    title: String,
    completed: bool
}

#[derive(Serialize, Deserialize)]
pub struct State {
    todo_list: Vec<Todo>
}

impl State {
    pub fn new() -> Self {
        State {
            todo_list: Vec::new()
        }
    }

    pub fn get_from_storage() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
            window().
            local_storage().
            get("state").
            and_then(|state_json| {
                serde_json::from_str(state_json.as_str()).ok()
            }).
            unwrap_or_else(State::new)))
    }

    pub fn save(&self) {
        // Save the state into local storage.
        let state_json = serde_json::to_string(&*self).unwrap();
        window().local_storage().insert("state", state_json.as_str());
    }
}

pub type StateRef = Rc<RefCell<State>>;
