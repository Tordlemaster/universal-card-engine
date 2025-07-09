use crate::rules::{game::GameWorld, state::StateSwitchData, variable::VarBindSet};

pub trait Routine {
    fn execute (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld) -> Option<StateSwitchData>;

    fn undo (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld) -> ();
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
    fn execute (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld) -> Option<StateSwitchData> {
        for r in self.routine.iter_mut() {
            if let Some(ssd) = r.execute(&bindings.clone(), game_world) {
                return Some(ssd);
            }
        }
        //None of the sub-routines returned a StateSwitchData
        None
    }

    fn undo (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld) -> () {
        for r in self.routine.iter_mut().rev() {
            r.undo(&bindings.clone(), game_world);
        }
    }
}