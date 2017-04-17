use std::cell::Cell;

use block::{Block, State};
use input::I3barEvent;

use serde_json::Value;

pub struct Toggle {
    pub state: Cell<State>,
    pub name: &'static str,
}

impl Toggle {
    pub fn new(name: &'static str) -> Toggle {
        Toggle {
            state: Cell::new(State::Idle),
            name: name,
        }
    }
}


impl Block for Toggle {
    fn id(&self) -> Option<&str> { Some(self.name) }

    fn click(&self, _: I3barEvent) {
        let s = self.state.get();
        use self::State::*;
        self.state.set(match s {
            Idle => Info,
            Info => Good,
            Good => Warning,
            Warning => Critical,
            Critical => Idle
        });
    }

    fn get_status(&self, _: &Value) -> Value {
        json!({"full_text": String::from("I can cycle state! Click me"),})
    }

    fn get_state(&self) -> State { self.state.get() }
}
