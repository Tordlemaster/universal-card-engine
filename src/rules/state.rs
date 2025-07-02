use super::{game::*, variable::*};

pub struct State {
    pub routine: fn(&mut GameWorld, &mut VarBindSet) -> StateSwitchData,
    pub bindings: VarBindSet,
}

impl State {
    pub fn switch_to (&mut self, bindings: VarBindSet) {
        self.bindings = bindings;
    }
}

pub struct StateSwitchData {
    next_state_name: String,
    bindings: VarBindSet
}

impl StateSwitchData {
    pub fn new (next_state_name: String, initial_bindings: VarBindSet) -> StateSwitchData {
        StateSwitchData {
            next_state_name: next_state_name,
            bindings: initial_bindings
        }
    }
}