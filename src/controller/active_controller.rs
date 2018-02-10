use controller::Controller;
use model::State;

pub struct ActiveController {
}

impl Controller<State> for ActiveController {
    fn navigate<'a>(&mut self, state: &'a State) {
        js! {
            console.log("active navigated");
        };
    }

    fn leave<'a>(&mut self, state: &'a State) {
        js! {
            console.log("active left");
        };
    }
}
