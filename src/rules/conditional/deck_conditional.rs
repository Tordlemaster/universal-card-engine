use std::cmp::{max, min};

use crate::rules::{conditional::conditional::{Conditional, ConditionalMode, ValCompMode}, deck::Deck, game::GameWorld, routine::evaluatables::EvaluatableString, variable::{TempVars, VarBindSet}};

pub trait DeckConditionalElement {
    fn evaluate(&self, deck: &Deck) -> bool;
}

pub struct DeckConditional {
    conditions: Vec<Box<dyn DeckConditionalElement>>,
    mode: ConditionalMode,
    deck_name: EvaluatableString
}

impl DeckConditional {
    pub fn new(conditions: Vec<Box<dyn DeckConditionalElement>>, mode: ConditionalMode, deck_name: &String) -> DeckConditional {
        DeckConditional {conditions: conditions, mode: mode, deck_name: EvaluatableString::new(deck_name)}
    }
}

impl Conditional for DeckConditional {
    fn evaluate(&self, bindings: &VarBindSet, game_world: &GameWorld, choice_vars: &mut TempVars) -> bool {
        let deck = game_world.get_deck(&self.deck_name.evaluate(bindings, game_world, choice_vars)).unwrap();
        if self.mode == ConditionalMode::And {
            let mut result = true;
            for c in &self.conditions {
                result = result && c.evaluate(deck);
                if !result { //Short circuit AND when value becomes false
                    break;
                }
            }
            result
        }
        else {
            let mut result = false;

            for c in &self.conditions {
                result = result || c.evaluate(deck);
                if result { //Short circuit OR when value becomes true
                    break;
                }
            }
            result
        }
    }
}


pub struct DeckLenConditional {
    len: usize,
    mode: ValCompMode
}

impl DeckLenConditional {
    pub fn new(len: usize, mode: ValCompMode) -> DeckLenConditional {
        DeckLenConditional { len: len, mode: mode }
    }
}

impl DeckConditionalElement for DeckLenConditional {
    fn evaluate(&self, deck: &Deck) -> bool {
        match self.mode {
            ValCompMode::Less => deck.len() < self.len,
            ValCompMode::LEq => deck.len() <= self.len,
            ValCompMode::Eq => deck.len() == self.len,
            ValCompMode::GEq => deck.len() >= self.len,
            ValCompMode::Greater => deck.len() > self.len
        }
    }
}

pub enum DeckSuitsComp {
    Same
}

pub struct DeckSuitsConditional{
    mode: DeckSuitsComp
}

impl DeckSuitsConditional {
    pub fn new(mode: DeckSuitsComp) -> DeckSuitsConditional {
        DeckSuitsConditional { mode: mode }
    }
}

impl DeckConditionalElement for DeckSuitsConditional {
    fn evaluate(&self, deck: &Deck) -> bool {
        match self.mode {
            DeckSuitsComp::Same => {
                if deck.len() > 0 {
                    let mut cards = deck.card_iter();
                    let suit = cards.next().unwrap().get_suit();
                    for c in cards {
                        if c.get_suit() != suit {
                            return false;
                        }
                    }
                    true
                }
                else {
                    true
                }
            }
        }
    }
}

pub enum DeckValsComp {
    Same,
    ///Consecutive
    Cons
}

pub struct DeckValsConditional {
    mode: DeckValsComp
}

impl DeckValsConditional {
    pub fn new(mode: DeckValsComp) -> DeckValsConditional {
        DeckValsConditional { mode: mode }
    }
}

impl DeckConditionalElement for DeckValsConditional {
    fn evaluate(&self, deck: &Deck) -> bool {
        match self.mode {
            DeckValsComp::Same => {
                if deck.len() > 0 {
                    let mut cards = deck.card_iter();
                    let val = cards.next().unwrap().get_value();
                    for c in cards {
                        if c.get_value() != val {
                            return false;
                        }
                    }
                    true
                }
                else {
                    true
                }
            },
            DeckValsComp::Cons => {
                if deck.len() > 0 {
                    let mut min_val = usize::MAX;
                    let mut max_val = 0_usize;

                    for c in deck.card_iter() {
                        min_val = min(min_val, c.get_value());
                        max_val = max(max_val, c.get_value());
                    }

                    if max_val - min_val + 1 != deck.len() {
                        return false;
                    }

                    let mut has_seen = vec![false; max_val - min_val + 1];

                    for c in deck.card_iter() {
                        if has_seen[c.get_value() - min_val] {
                            return false;
                        }
                        has_seen[c.get_value() - min_val] = true;
                    }

                    true
                }
                else {
                    false
                }
            }
        }
    }
}