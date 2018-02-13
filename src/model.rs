use std::cell::RefCell;
use std::rc::Rc;

use serde_json;

use stdweb::web::window;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Todo {
    pub title: String,
    pub completed: bool
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub todo_list: Vec<Todo>
}

impl State {
    pub fn new() -> Self {
        State {
            todo_list: Vec::new()
        }
    }

    pub fn get_from_storage() -> Self {
        window().
        local_storage().
        get("state").
        and_then(|state_json| {
            serde_json::from_str(state_json.as_str()).ok()
        }).
        unwrap_or_else(State::new)
    }

    pub fn save(&self) {
        // Save the state into local storage.
        let state_json = serde_json::to_string(&*self).unwrap();
        window().local_storage().insert("state", state_json.as_str());
    }
}
