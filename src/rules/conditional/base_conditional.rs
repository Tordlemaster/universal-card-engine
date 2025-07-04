use crate::rules::game::GameWorld;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ConditionalMode {
    And,
    Or
}

pub trait Conditional {
    fn evaluate(&self, game_world: &GameWorld) -> bool;
}

pub struct BaseConditional {
    conditions: Vec<Box<dyn Conditional>>,
    mode: ConditionalMode
}

impl BaseConditional {
    pub fn new(conditions: Vec<Box<dyn Conditional>>, mode: ConditionalMode) -> BaseConditional {
        BaseConditional { conditions: conditions, mode: mode }
    }
}

impl Conditional for BaseConditional {
    fn evaluate(&self, game_world: &GameWorld) -> bool {
        if self.mode == ConditionalMode::And {
            let mut result = true;
            for c in &self.conditions {
                result = result && c.evaluate(game_world);
                if !result { //Short circuit AND when value becomes false
                    break;
                }
            }
            result
        }
        else {
            let mut result = false;
            for c in &self.conditions {
                result = result || c.evaluate(game_world);
                if result { //Short circuit OR when value becomes true
                    break;
                }
            }
            result
        }
    }
}