use crate::rules::{conditional::conditional::{Conditional, TrueConditional}, routine::routine::Routine, state::StateSwitchData};

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

    pub fn execute (&self, bindings: &crate::rules::variable::VarBindSet, game_world: &mut crate::rules::game::GameWorld) -> CondRoutineReturn {
        match self.mode {
            CondRoutineMode::PreCond => {
                if self.cond.evaluate(bindings, game_world) {
                    return CondRoutineReturn::Success(self.routine.execute(bindings, game_world));
                }
                else {
                    return CondRoutineReturn::Failure("Failed to meet the action's conditions".to_string());
                }
            }
            CondRoutineMode::PostCond => {
                let ret = CondRoutineReturn::Success(self.routine.execute(bindings, game_world));
                if !self.cond.evaluate(bindings, game_world) {
                    self.routine.undo(bindings, game_world);
                    return CondRoutineReturn::Failure("Failed to meet the action's conditions".to_string());
                }
                else {
                    return ret;
                }
            }
        }
    }
    pub fn undo (&self, bindings: &crate::rules::variable::VarBindSet, game_world: &mut crate::rules::game::GameWorld) -> () {
        self.routine.undo(bindings, game_world);
        //TODO??
    }
}