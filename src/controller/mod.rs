use stdweb::web::event::{IEvent, ConcreteEvent};
use std::error::Error;
use stdweb::unstable::Void;

pub trait Controller<State> {
    // FIXME: return values
    // navigate sets up event listeners
    fn navigate<'a>(&self, state: &'a mut State);
    
    // leave tears down event listeners
    fn leave<'a>(&self, state: &'a mut State);
}
