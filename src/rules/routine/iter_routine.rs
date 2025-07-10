use crate::rules::{conditional::conditional::Conditional, game::GameWorld, routine::routine::Routine, state::StateSwitchData, variable::{TempVars, VarBindSet}};

pub struct ForPlayerRoutine {
    routine: Box<dyn Routine>
}

impl ForPlayerRoutine {
    pub fn new(routine: Box<dyn Routine>) -> ForPlayerRoutine {
        ForPlayerRoutine { routine: routine }
    }
}

impl Routine for ForPlayerRoutine {
    fn execute (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> Option<StateSwitchData> {
        let players = game_world.get_players().clone();
        for player in players.players_iter() {

            println!("\n-- {}'s Turn --\n", player.name());

            let mut new_bindings = bindings.clone();
            new_bindings.insert_str_var(&String::from("THISPLAYER"), player.name().clone());

            let s = self.routine.execute(&new_bindings, game_world, choice_vars);
            if s.is_some() {
                return s;
            }
        }
        None
    }

    fn undo (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> () {
        let players = game_world.get_players().clone();
        for player in players.players_iter() {

            let mut new_bindings = bindings.clone();
            new_bindings.insert_str_var(&String::from("THISPLAYER"), player.name().clone());

            self.routine.undo(&new_bindings, game_world, choice_vars);
        }
    }
}

pub struct ForPlayerCondRoutine {
    routine: Box<dyn Routine>,
    cond: Box<dyn Conditional>
}

impl ForPlayerCondRoutine {
    pub fn new(cond: Box<dyn Conditional>, routine: Box<dyn Routine>) -> ForPlayerCondRoutine {
        ForPlayerCondRoutine { routine: routine, cond: cond}
    }
}

impl Routine for ForPlayerCondRoutine {
    fn execute (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> Option<StateSwitchData> {
        let players = game_world.get_players().clone();
        for player in players.players_iter() {

            println!("\n-- {}'s Turn --\n", player.name());

            let mut new_bindings = bindings.clone();
            new_bindings.insert_str_var(&String::from("THISPLAYER"), player.name().clone());

            let s = self.routine.execute(&new_bindings, game_world, choice_vars);
            if s.is_some() {
                return s;
            }
        }
        None
    }

    fn undo (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> () {
        let players = game_world.get_players().clone();
        for player in players.players_iter() {

            let mut new_bindings = bindings.clone();
            new_bindings.insert_str_var(&String::from("THISPLAYER"), player.name().clone());

            self.routine.undo(&new_bindings, game_world, choice_vars);
        }
    }
}