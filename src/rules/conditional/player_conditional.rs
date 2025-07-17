use crate::rules::{conditional::conditional::{Conditional, ConditionalMode, ValCompMode}, game::GameWorld, player::Player, routine::evaluatables::EvaluatableString, variable::{TempVars, VarBindSet}};

pub trait PlayerConditionalElement {
    fn evaluate(&self, player: &Player, bindings: &VarBindSet, game_world: &GameWorld, choice_var: &mut TempVars) -> bool;
}

pub struct PlayerConditional {
    conditions: Vec<Box<dyn PlayerConditionalElement>>,
    mode: ConditionalMode,
    player_var: EvaluatableString
}

impl PlayerConditional {
    pub fn new(conditions: Vec<Box<dyn PlayerConditionalElement>>, mode: ConditionalMode, player_var: &String) -> PlayerConditional {
        PlayerConditional { conditions: conditions, mode: mode, player_var: EvaluatableString::new(player_var) }
    }
}

impl Conditional for PlayerConditional {
    fn evaluate(&self, bindings: &VarBindSet, game_world: &GameWorld, choice_vars: &mut TempVars) -> bool {
        let player = game_world.get_players().get_player_by_name(&self.player_var.evaluate(bindings, game_world, choice_vars)).unwrap();
        if self.mode == ConditionalMode::And {
            let mut result = true;
            for c in &self.conditions {
                result = result && c.evaluate(&player, bindings, game_world, choice_vars);
                if !result { //Short circuit AND when value becomes false
                    break;
                }
            }
            result
        }
        else {
            let mut result = false;
            for c in &self.conditions {
                result = result || c.evaluate(&player, bindings, game_world, choice_vars);
                if result { //Short circuit OR when value becomes true
                    break;
                }
            }
            result
        }
    }
}

pub struct PlayerNameConditional {
    name: EvaluatableString
}

impl PlayerNameConditional {
    pub fn new(name: &String) -> PlayerNameConditional {
        PlayerNameConditional { name: EvaluatableString::new(name) }
    }
}

impl PlayerConditionalElement for PlayerNameConditional {
    fn evaluate(&self, player: &Player, bindings: &VarBindSet, game_world: &GameWorld, choice_var: &mut TempVars) -> bool {
        *player.name() == self.name.evaluate(bindings, game_world, choice_var)
    }
}

pub struct PlayerScoreConditional {
    val: u32,
    mode: ValCompMode
}

impl PlayerScoreConditional {
    pub fn new(val: u32, mode: ValCompMode) -> PlayerScoreConditional {
        PlayerScoreConditional { val: val, mode: mode }
    }
}

impl PlayerConditionalElement for PlayerScoreConditional {
    fn evaluate(&self, player: &Player, bindings: &VarBindSet, game_world: &GameWorld, choice_var: &mut TempVars) -> bool {
        let score = player.score();
        match self.mode {
            ValCompMode::Less => score < self.val,
            ValCompMode::LEq => score <= self.val,
            ValCompMode::Eq => score == self.val,
            ValCompMode::GEq => score >= self.val,
            ValCompMode::Greater => score > self.val
        }
    }
}