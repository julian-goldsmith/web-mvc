use controller::Controller;
use model::State;

pub struct ActiveController {
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
