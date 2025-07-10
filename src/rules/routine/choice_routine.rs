use crate::{interface::interface::choice_interface, rules::{conditional::conditional::TrueConditional, game::GameWorld, routine::{cond_routine::{CondRoutine, CondRoutineMode, CondRoutineReturn}, primitives::NullRoutine, routine::Routine}, state::StateSwitchData, variable::{TempVars, VarBindSet}}};

#[derive(Clone, Copy, PartialEq, Eq)]
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

    pub fn execute (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> CondRoutineReturn {
        self.routine.execute(bindings, game_world, choice_vars)
    }
    pub fn undo (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> () {
        self.routine.undo(bindings, game_world, choice_vars)
    }
}

pub struct ChoicesRoutine {
    choices: Vec<Choice>,
    mode: ChoiceLimit
}

impl ChoicesRoutine {
    pub fn new(mut choices: Vec<Choice>, mode: ChoiceLimit) -> ChoicesRoutine {
        if mode == ChoiceLimit::Unlimited {
            choices.push((Choice::new("End turn".to_string(), CondRoutine::new(Box::new(TrueConditional), Box::new(NullRoutine), CondRoutineMode::PreCond))));
        }
    
        ChoicesRoutine { choices: choices, mode: mode }
    }
}

impl Routine for ChoicesRoutine {
    fn execute (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> Option<StateSwitchData> {
        match self.mode {
            ChoiceLimit::Limited(limit) => {
                for l in (1..=limit).rev() {
                    let mut success = false;
                    while !success {
                        println!("Choose one of the following ({} of {} actions remaining):", l, limit);
                        let u = choice_interface(&self.choices);
                        let mut temp_vars = TempVars::new();
                        match self.choices[u].execute(bindings, game_world, &mut temp_vars) {
                            CondRoutineReturn::Success(a) => {
                                success = true;
                                return a;
                            },
                            CondRoutineReturn::Failure(a) => {
                                println!("Condition for action not satisfied: {}\nPlease choose a different action.", a);
                                self.choices[u].undo(bindings, game_world, &mut temp_vars);
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
                    //println!("i = {} choices.len() = {}", i, self.choices.len());
                    if i == self.choices.len() - 1 {
                        stop = true;
                    }
                    else {
                        let mut success = false;
                        while !success {
                            let mut temp_vars = TempVars::new();
                            match self.choices[i].execute(bindings, game_world, &mut temp_vars) {
                                CondRoutineReturn::Success(a) => {
                                    success = true;
                                    if a.is_some() {
                                        return a;
                                    }
                                },
                                CondRoutineReturn::Failure(a) => {
                                    println!("Condition for action not satisfied: {}\nPlease choose a different action.", a);
                                    self.choices[i].undo(bindings, game_world, &mut temp_vars);
                                }
                            }
                        }
                    }
                }
            }
        }

        None
    }
    fn undo (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> () {
        //TODO
    }
}