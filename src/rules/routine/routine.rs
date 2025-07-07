use crate::rules::{game::GameWorld, state::StateSwitchData, variable::VarBindSet};

pub trait Routine {
    fn execute (&self, bindings: &VarBindSet, game_world: &mut GameWorld) -> Option<StateSwitchData>;

    fn undo (&self, bindings: &VarBindSet, game_world: &mut GameWorld) -> ();
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
    fn execute (&self, bindings: &VarBindSet, game_world: &mut GameWorld) -> Option<StateSwitchData> {
        for r in self.routine.iter() {
            if let Some(ssd) = r.execute(&bindings.clone(), game_world) {
                return Some(ssd);
            }
        }
        //None of the sub-routines returned a StateSwitchData
        None
    }

    fn undo (&self, bindings: &VarBindSet, game_world: &mut GameWorld) -> () {
        for r in self.routine.iter().rev() {
            r.undo(&bindings.clone(), game_world);
        }
    }
}