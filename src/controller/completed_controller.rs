use controller::Controller;
use model::State;

pub struct CompletedController {
}

impl Controller<State> for CompletedController {
    fn navigate<'a>(&mut self, state: &'a mut State) {
        js! {
            console.log("completed navigated");
        };
    }

    fn leave<'a>(&mut self, state: &'a mut State) {
        js! {
            console.log("completed left");
        };
    }
}

