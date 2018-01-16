use controller::Controller;
use model::State;

pub struct RootController {
}

impl Controller<State> for RootController {
    fn navigate<'a>(&self, state: &'a mut State) {
        js! {
            console.log("root navigated");
        };
    }

    fn leave<'a>(&self, state: &'a mut State) {
        js! {
            console.log("root left");
        };
    }
}
