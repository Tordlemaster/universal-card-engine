use std::cmp::{max, min};

use crate::rules::{conditional::base_conditional::{Conditional, ConditionalMode}, deck::Deck};

pub trait DeckConditionalElement {
    fn evaluate(&self, deck: &Deck) -> bool;
}

pub struct DeckConditional {
    conditions: Vec<Box<dyn DeckConditionalElement>>,
    mode: ConditionalMode,
    deck_name: String
}

impl DeckConditional {
    pub fn new(conditions: Vec<Box<dyn DeckConditionalElement>>, mode: ConditionalMode, deck_name: String) -> DeckConditional {
        DeckConditional {conditions: conditions, mode: mode, deck_name: deck_name}
    }
}

impl Conditional for DeckConditional {
    fn evaluate(&self, game_world: &crate::rules::game::GameWorld) -> bool {
        if self.mode == ConditionalMode::And {
            let mut result = true;
            for c in &self.conditions {
                result = result && c.evaluate(game_world.get_deck(&self.deck_name).unwrap());
                if !result { //Short circuit AND when value becomes false
                    break;
                }
            }
            result
        }
        else {
            let mut result = false;
            for c in &self.conditions {
                result = result || c.evaluate(game_world.get_deck(&self.deck_name).unwrap());
                if result { //Short circuit OR when value becomes true
                    break;
                }
            }
            result
        }
    }
}

pub enum DeckLenComp {
    Less, LEq, Eq, GEq, Greater
}
pub struct DeckLenConditional {
    len: usize,
    mode: DeckLenComp
}

impl DeckConditionalElement for DeckLenConditional {
    fn evaluate(&self, deck: &Deck) -> bool {
        match self.mode {
            DeckLenComp::Less => deck.len() < self.len,
            DeckLenComp::LEq => deck.len() <= self.len,
            DeckLenComp::Eq => deck.len() == self.len,
            DeckLenComp::GEq => deck.len() >= self.len,
            DeckLenComp::Greater => deck.len() > self.len
        }
    }
}

pub enum DeckSuitsComp {
    Same
}

pub struct DeckSuitsConditional{
    mode: DeckSuitsComp
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
    Same, Consecutive
}

pub struct DeckValsConditional {
    mode: DeckValsComp
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
            DeckValsComp::Consecutive => {
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