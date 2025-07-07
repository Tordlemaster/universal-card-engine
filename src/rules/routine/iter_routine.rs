use crate::rules::{game::GameWorld, routine::routine::Routine, state::StateSwitchData, variable::VarBindSet};

pub struct ForPlayerRoutine {
    routine: Box<dyn Routine>
}

impl ForPlayerRoutine {
    pub fn new(routine: Box<dyn Routine>) -> ForPlayerRoutine {
        ForPlayerRoutine { routine: routine }
    }
}

impl Routine for ForPlayerRoutine {
    fn execute (&self, bindings: &VarBindSet, game_world: &mut GameWorld) -> Option<StateSwitchData> {
        let players = game_world.get_players().clone();
        for i in 0..players.num_players() {
            let player = players.get_player(i);

            let mut new_bindings = bindings.clone();
            new_bindings.insert_str_var(&String::from("THISPLAYER"), player.name().clone());

            let s = self.routine.execute(&new_bindings, game_world);
            if s.is_some() {
                return s;
            }
        }
        None
    }

    fn undo (&self, bindings: &VarBindSet, game_world: &mut GameWorld) -> () {
        let players = game_world.get_players().clone();
        for i in 0..players.num_players() {
            let player = players.get_player(i);

            let mut new_bindings = bindings.clone();
            new_bindings.insert_str_var(&String::from("THISPLAYER"), player.name().clone());

            self.routine.undo(&new_bindings, game_world);
        }
    }
}