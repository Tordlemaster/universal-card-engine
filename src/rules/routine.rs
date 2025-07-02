use crate::rules::{game::GameWorld, variable::VarBindSet};

pub trait Routine {
    fn execute (&self, bindings: VarBindSet, game_world: &mut GameWorld) -> ();

    fn undo (&self, bindings: VarBindSet, game_world: &mut GameWorld) -> ();
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
    fn execute (&self, bindings: VarBindSet, game_world: &mut GameWorld) -> () {
        for r in self.routine.iter() {
            r.execute(bindings.clone(), game_world);
        }
    }

    fn undo (&self, bindings: VarBindSet, game_world: &mut GameWorld) -> () {
        for r in self.routine.iter().rev() {
            r.undo(bindings.clone(), game_world);
        }
    }
}

pub struct ForPlayerRoutine {
    routine: Vec<Box<dyn Routine>>
}

impl Routine for ForPlayerRoutine {
    fn execute (&self, bindings: VarBindSet, game_world: &mut GameWorld) -> () {
        for player in game_world.players {

            let mut new_bindings = bindings.clone();
            new_bindings.insert_str_var(String::from("THISPLAYER"), player.name);

            for r in self.routine.iter() {
                r.execute(new_bindings, game_world);
            }
        }
    }

    fn undo (&self, bindings: VarBindSet, game_world: &mut GameWorld) -> () {
        for player in game_world.players {

            let mut new_bindings = bindings.clone();
            new_bindings.insert_str_var(String::from("THISPLAYER"), player.name);

            for r in self.routine.iter().rev() {
                r.undo(new_bindings, game_world);
            }
        }
    }
}