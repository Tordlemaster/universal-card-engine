use crate::rules::{game::GameWorld, variable::VarBindSet};


#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ConditionalMode {
    And,
    Or
}

pub trait Conditional {
    fn evaluate(&self, bindings: &VarBindSet, game_world: &GameWorld) -> bool;
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
    fn evaluate(&self, bindings: &VarBindSet, game_world: &GameWorld) -> bool {
        if self.mode == ConditionalMode::And {
            let mut result = true;
            for c in &self.conditions {
                result = result && c.evaluate(bindings, game_world);
                if !result { //Short circuit AND when value becomes false
                    break;
                }
            }
            result
        }
        else {
            let mut result = false;
            for c in &self.conditions {
                result = result || c.evaluate(bindings, game_world);
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
    fn evaluate(&self, bindings: &VarBindSet, game_world: &GameWorld) -> bool {
        true
    }
}