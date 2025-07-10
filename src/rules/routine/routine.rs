use crate::rules::{game::GameWorld, state::StateSwitchData, variable::{TempVars, VarBindSet}};

pub trait Routine {
    fn execute (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> Option<StateSwitchData>;

    fn undo (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> ();
}

pub struct SeqRoutine {
    routine: Vec<Box<dyn Routine>>,
}

impl SeqRoutine {
    pub fn new (routine: Vec<Box<dyn Routine>>) -> SeqRoutine {
        SeqRoutine {
            routine: routine,
        }
    }
}

impl Routine for SeqRoutine {
    fn execute (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> Option<StateSwitchData> {
        for r in self.routine.iter_mut() {
            if let Some(ssd) = r.execute(&bindings.clone(), game_world, choice_vars) {
                return Some(ssd);
            }
        }
        //None of the sub-routines returned a StateSwitchData
        None
    }

    fn undo (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> () {
        for r in self.routine.iter_mut().rev() {
            r.undo(&bindings.clone(), game_world, choice_vars);
        }
    }
}