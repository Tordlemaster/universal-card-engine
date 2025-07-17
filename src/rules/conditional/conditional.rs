use crate::rules::{game::GameWorld, variable::{TempVars, VarBindSet}};

pub enum ValCompMode {
    Less, LEq, Eq, GEq, Greater
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ConditionalMode {
    And,
    Or
}

pub trait Conditional {
    fn evaluate(&self, bindings: &VarBindSet, game_world: &GameWorld, choice_vars: &mut TempVars) -> bool;
}

pub struct MultiConditional {
    conditions: Vec<Box<dyn Conditional>>,
    mode: ConditionalMode
}

impl MultiConditional {
    pub fn new(conditions: Vec<Box<dyn Conditional>>, mode: ConditionalMode) -> MultiConditional {
        MultiConditional { conditions: conditions, mode: mode }
    }
}

impl Conditional for MultiConditional {
    fn evaluate(&self, bindings: &VarBindSet, game_world: &GameWorld, choice_vars: &mut TempVars) -> bool {
        if self.mode == ConditionalMode::And {
            let mut result = true;
            for c in &self.conditions {
                result = result && c.evaluate(bindings, game_world, choice_vars);
                if !result { //Short circuit AND when value becomes false
                    break;
                }
            }
            result
        }
        else {
            let mut result = false;
            for c in &self.conditions {
                result = result || c.evaluate(bindings, game_world, choice_vars);
                if result { //Short circuit OR when value becomes true
                    break;
                }
            }
            result
        }
    }
}

pub struct TrueConditional;

impl Conditional for TrueConditional {
    fn evaluate(&self, bindings: &VarBindSet, game_world: &GameWorld, choice_vars: &mut TempVars) -> bool {
        true
    }
}

pub struct NotConditional {
    condition: Box<dyn Conditional>
}

impl NotConditional {
    pub fn new(condition: Box<dyn Conditional>) -> NotConditional {
        NotConditional { condition: condition }
    }
}

impl Conditional for NotConditional {
    fn evaluate(&self, bindings: &VarBindSet, game_world: &GameWorld, choice_vars: &mut TempVars) -> bool {
        !self.condition.evaluate(bindings, game_world, choice_vars)
    }
}