use std::cell::RefCell;
use std::rc::Rc;

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
}

pub type StateRef = Rc<RefCell<State>>;
