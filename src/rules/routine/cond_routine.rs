use crate::rules::{conditional::conditional::{Conditional, TrueConditional}, game::GameWorld, routine::routine::Routine, state::StateSwitchData, variable::{TempVars, VarBindSet}};

pub enum CondRoutineReturn {
    Success(Option<StateSwitchData>),

    ///Message to inform the player why their choice of action failed
    Failure(String)
}

pub enum CondRoutineMode {
    ///Evaluate the condition before executing the routine
    PreCond,

    ///Evaluate the condition after executing the routine and undo it if the condition is not met
    PostCond
}

pub struct CondRoutine {
    cond: Box<dyn Conditional>,
    routine: Box<dyn Routine>,
    mode: CondRoutineMode
}

impl CondRoutine {
    pub fn new(cond: Box<dyn Conditional>, routine: Box<dyn Routine>, mode: CondRoutineMode) -> CondRoutine {
        CondRoutine { cond: cond, routine: routine, mode: mode }
    }

    ///Shortcut for tnitializing a CondRoutine that will always execute and never fail
    pub fn without_cond(routine: Box<dyn Routine>) -> CondRoutine {
        CondRoutine { cond: Box::new(TrueConditional), routine: routine, mode: CondRoutineMode::PostCond }
    }

    pub fn execute (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> CondRoutineReturn {
        match self.mode {
            CondRoutineMode::PreCond => {
                if self.cond.evaluate(bindings, game_world, choice_vars) {
                    return CondRoutineReturn::Success(self.routine.execute(bindings, game_world, choice_vars));
                }
                else {
                    return CondRoutineReturn::Failure("Failed to meet the action's conditions".to_string());
                }
            }
            CondRoutineMode::PostCond => {
                let ret = CondRoutineReturn::Success(self.routine.execute(bindings, game_world, choice_vars));
                if !self.cond.evaluate(bindings, game_world, choice_vars) {
                    return CondRoutineReturn::Failure("Failed to meet the action's conditions".to_string());
                }
                else {
                    return ret;
                }
            }
        }
    }
    pub fn undo (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> () {
        self.routine.undo(bindings, game_world, choice_vars);
        //TODO??
    }
}

pub struct IfRoutine {
    cr: CondRoutine
}

impl IfRoutine {
    pub fn new(cr: CondRoutine) -> IfRoutine {
        IfRoutine { cr: cr }
    }
}

impl Routine for IfRoutine {
    fn execute (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> Option<StateSwitchData> {
        if let CondRoutineReturn::Success(ret) = self.cr.execute(bindings, game_world, choice_vars) {
            ret
        }
        else {
            None
        }
    }
    fn undo (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> () {
        self.cr.undo(bindings, game_world, choice_vars);
    }
}