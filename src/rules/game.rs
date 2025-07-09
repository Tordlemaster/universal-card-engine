use std::{cmp::max, collections::{hash_map::Iter, HashMap}};

use crate::rules::{deck::{CardSetData, Deck, DeckSet, DeckVisibility}, player::{Player, PlayerSet}, state::StateSet};

pub struct GameWorld {
    decks: DeckSet,
    players: PlayerSet,
    card_set_data: CardSetData,
}
pub struct Game {
    world: GameWorld,
    states: StateSet
}

impl Game {
    pub fn new(players: Vec<Player>, card_set_data: CardSetData, state_set: StateSet) -> Game {
        Game { world: GameWorld{decks: DeckSet::new(), players: PlayerSet::new(players), card_set_data: card_set_data}, states: state_set}
    }

    pub fn launch(&mut self) {
        self.states.launch(&mut self.world);
    }

    pub fn world(&self) -> &GameWorld {
        &self.world
    }

    pub fn add_source_deck(&mut self, name: String, visibility: DeckVisibility) {
        self.world.decks.add_source_deck(self.world.card_set_data.total_cards(), name, visibility);
    }
    pub fn add_deck(&mut self, name: String, visibility: DeckVisibility) {
        self.world.decks.add_deck(name, visibility);
    }

    pub fn remove_deck(&mut self, name: &String) {
        self.world.decks.remove_deck(name);
    }

    pub fn deal(&mut self, source: &String, dest: &String, n: usize) -> Result<usize, usize>{
        for i in 0..n {
            if let Some(source_deck) = self.world.decks.get_deck_mut(source) {
                if let Some(card) = source_deck.draw_card() {
                    if let Some(dest_deck) = self.world.decks.get_deck_mut(dest) {
                        dest_deck.insert_card(card);
                    }
                    else {
                        panic!("Script error: Deck \"{}\" not found", dest);
                    }
                }
                else {
                    return Err(max(i-1, 0));
                }
            }
            else {
                panic!("Script error: Deck \"{}\" not found", source);
            }
        }
        return Ok(n);
    }

    //Deal card at index idx in source deck to dest deck
    pub fn deal_idx(&mut self, source: &String, dest: &String, idx: usize) -> Result<usize, usize>{
        if let Some(source_deck) = self.world.decks.get_deck_mut(source) {
            if let Some(card) = source_deck.draw_card_idx(idx) {
                if let Some(dest_deck) = self.world.decks.get_deck_mut(dest) {
                    dest_deck.insert_card(card);
                }
                else {
                    panic!("Script error: Deck \"{}\" not found", dest);
                }
            }
            else {
                return Err(0);
            }
        }
        else {
            panic!("Script error: Deck \"{}\" not found", source);
        }
        return Ok(1);
    }

    pub fn get_deck(&self, name: &String) -> Option<&Deck> {
        self.world.decks.get_deck(name)
    }

    pub fn get_card_set_data(&self) -> &CardSetData {
        &self.world.card_set_data
    }

    pub fn get_decks(&self) -> Iter<String, Deck> {
        self.world.decks.iter_decks()
    }

    pub fn get_players(&self) -> &PlayerSet {
        &self.world.players
    }
}

impl GameWorld {

    pub fn add_source_deck(&mut self, name: String, visibility: DeckVisibility) {
        self.decks.add_source_deck(self.card_set_data.total_cards(), name, visibility);
    }
    pub fn add_deck(&mut self, name: String, visibility: DeckVisibility) {
        self.decks.add_deck(name, visibility);
    }

    pub fn remove_deck(&mut self, name: &String) {
        self.decks.remove_deck(name);
    }

    pub fn deal(&mut self, source: &String, dest: &String, n: usize) -> Result<usize, usize>{
        for i in 0..n {
            if let Some(source_deck) = self.decks.get_deck_mut(source) {
                if let Some(card) = source_deck.draw_card() {
                    if let Some(dest_deck) = self.decks.get_deck_mut(dest) {
                        dest_deck.insert_card(card);
                    }
                    else {
                        panic!("Script error: Deck \"{}\" not found", dest);
                    }
                }
                else {
                    return Err(max(i-1, 0));
                }
            }
            else {
                panic!("Script error: Deck \"{}\" not found", source);
            }
        }
        return Ok(n);
    }

    //Deal card at index idx in source deck to dest deck
    pub fn deal_idx(&mut self, source: &String, dest: &String, idx: usize) -> Result<usize, usize>{
        if let Some(source_deck) = self.decks.get_deck_mut(source) {
            if let Some(card) = source_deck.draw_card_idx(idx) {
                if let Some(dest_deck) = self.decks.get_deck_mut(dest) {
                    dest_deck.insert_card(card);
                }
                else {
                    panic!("Script error: Deck \"{}\" not found", dest);
                }
            }
            else {
                return Err(0);
            }
        }
        else {
            panic!("Script error: Deck \"{}\" not found", source);
        }
        return Ok(1);
    }

    pub fn get_deck(&self, name: &String) -> Option<&Deck> {
        self.decks.get_deck(name)
    }

    pub fn get_card_set_data(&self) -> &CardSetData {
        &self.card_set_data
    }

    pub fn get_decks(&self) -> Iter<String, Deck> {
        self.decks.iter_decks()
    }

    pub fn get_players(&self) -> &PlayerSet {
        &self.players
    }
}