use std::collections::HashMap;

use crate::rules::routine::routine::Routine;

use super::{game::*, variable::*};

pub struct State {
    pub routine: Box<dyn Routine>,
}

//State's "execute" function should return with StateSwitchDate corresponding with the next state to switch to
impl State {
    pub fn new(routine: Box<dyn Routine>) -> State {
        State {routine: routine}
    }
    pub fn execute (&mut self, initial_bindings: VarBindSet, game_world: &mut GameWorld)  -> StateSwitchData {
        self.routine.execute(&initial_bindings.clone(), game_world, &mut TempVars::new()).unwrap()
    }
}

pub struct StateSet {
    states: HashMap<String, State>
}

impl StateSet {
    pub fn new(names: Vec<String>, mut states: Vec<Option<State>>) -> StateSet {
        let mut states_map = HashMap::new();
        for i in 0..names.len() {
            states_map.insert(names[i].clone(), states[i].take().unwrap());
        }

        StateSet { states: states_map }
    }

    pub fn launch(&mut self, game_world: &mut GameWorld) {
        let mut next_state = StateSwitchData { next_state_name: Some("SETUP".to_string()), bindings: VarBindSet::new() };
        while let Some(ref next) = next_state.next_state_name {
            if let Some(cur_state) = self.states.get_mut(next) {
                next_state = cur_state.execute(next_state.bindings, game_world);
            }
            else {
                panic!("Script error: no {} state found", next);
            }
        }
    }
}

pub struct StateSwitchData {
    next_state_name: Option<String>,
    bindings: VarBindSet
}

impl StateSwitchData {
    pub fn new (next_state_name: String, initial_bindings: VarBindSet) -> StateSwitchData {
        StateSwitchData {
            next_state_name: Some(next_state_name),
            bindings: initial_bindings
        }
    }
}