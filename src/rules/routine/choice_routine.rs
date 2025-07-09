use crate::{interface::interface::choice_interface, rules::{conditional::conditional::TrueConditional, routine::{cond_routine::{CondRoutine, CondRoutineMode, CondRoutineReturn}, primitives::NullRoutine, routine::Routine}}};

#[derive(Clone, Copy)]
pub enum ChoiceLimit {
    Limited(usize),
    Unlimited
}

pub struct ChoiceMode {
    ///The size of the subset of options from the list the player is allowed to select
    limit: ChoiceLimit
}

pub struct Choice {
    name: String,
    routine: CondRoutine
}

impl Choice {
    pub fn new(name: String, routine: CondRoutine) -> Choice {
        Choice { name: name, routine: routine }
    }
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn execute (&mut self, bindings: &crate::rules::variable::VarBindSet, game_world: &mut crate::rules::game::GameWorld) -> CondRoutineReturn {
        self.routine.execute(bindings, game_world)
    }
    pub fn undo (&mut self, bindings: &crate::rules::variable::VarBindSet, game_world: &mut crate::rules::game::GameWorld) -> () {
        self.routine.undo(bindings, game_world);
    }
}

pub struct ChoicesRoutine {
    choices: Vec<Choice>,
    mode: ChoiceLimit
}

impl ChoicesRoutine {
    pub fn new(mut choices: Vec<Choice>, mode: ChoiceLimit) -> ChoicesRoutine {
        choices.push((Choice::new("End turn".to_string(), CondRoutine::new(Box::new(TrueConditional), Box::new(NullRoutine), CondRoutineMode::PreCond))));
    
        ChoicesRoutine { choices: choices, mode: mode }
    }
}

impl Routine for ChoicesRoutine {
    fn execute (&mut self, bindings: &crate::rules::variable::VarBindSet, game_world: &mut crate::rules::game::GameWorld) -> Option<crate::rules::state::StateSwitchData> {
        match self.mode {
            ChoiceLimit::Limited(limit) => {
                for l in (1..=limit).rev() {
                    let mut success = false;
                    while !success {
                        println!("Choose one of the following ({} of {} actions remaining):", l, limit);
                        let u = choice_interface(&self.choices);
                        match self.choices[u].execute(bindings, game_world) {
                            CondRoutineReturn::Success(a) => {
                                success = true;
                                return a;
                            },
                            CondRoutineReturn::Failure(a) => {
                                self.choices[u].undo(bindings, game_world);
                                println!("Condition for action not satisfied: {}\nPlease choose a different action.", a);
                            }
                        }
                    }
                }
            }
            ChoiceLimit::Unlimited => {
                println!("Choose one of the following (unlimited actions):");
                let mut stop = false;
                while !stop {
                    let i = choice_interface(&self.choices);
                    if i == self.choices.len() {
                        stop = true;
                    }
                    else {
                        let mut success = false;
                        while !success {
                            match self.choices[i].execute(bindings, game_world) {
                                CondRoutineReturn::Success(a) => {
                                    success = true;
                                    return a;
                                },
                                CondRoutineReturn::Failure(a) => {
                                    self.choices[i].undo(bindings, game_world);
                                    println!("Condition for action not satisfied: {}\nPlease choose a different action.", a);
                                }
                            }
                        }
                    }
                }
            }
        }

        None
    }
    fn undo (&mut self, bindings: &crate::rules::variable::VarBindSet, game_world: &mut crate::rules::game::GameWorld) -> () {
        
    }
}