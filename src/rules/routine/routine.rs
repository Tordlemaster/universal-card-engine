use crate::rules::{game::GameWorld, variable::VarBindSet};

pub trait Routine {
    fn execute (&self, bindings: &VarBindSet, game_world: &mut GameWorld) -> ();

    fn undo (&self, bindings: &VarBindSet, game_world: &mut GameWorld) -> ();
}

pub struct BaseRoutine {
    routine: Vec<Box<dyn Routine>>,
    bindings: VarBindSet
}

impl BaseRoutine {
    pub fn new (routine: Vec<Box<dyn Routine>>, bindings: VarBindSet) -> BaseRoutine {
        BaseRoutine {
            routine: routine,
            bindings: bindings
        }
    }
}

impl Routine for BaseRoutine {
    fn execute (&self, bindings: &VarBindSet, game_world: &mut GameWorld) -> () {
        for r in self.routine.iter() {
            r.execute(&bindings.clone(), game_world);
        }
    }

    fn undo (&self, bindings: &VarBindSet, game_world: &mut GameWorld) -> () {
        for r in self.routine.iter().rev() {
            r.undo(&bindings.clone(), game_world);
        }
    }
}