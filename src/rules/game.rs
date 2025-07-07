use std::{cmp::max, collections::{hash_map::Iter, HashMap}};

use crate::rules::{deck::{CardSetData, Deck, DeckSet, DeckVisibility}, player::{Player, PlayerSet}, state::StateSet};

pub struct GameWorld {
    decks: DeckSet,
    players: PlayerSet,
    card_set_data: CardSetData,

    states: StateSet
}

impl GameWorld {
    pub fn new(players: Vec<Player>, card_set_data: CardSetData, state_set: StateSet) -> GameWorld {
        GameWorld { decks: DeckSet::new(), players: PlayerSet::new(players), card_set_data: card_set_data, states: state_set}
    }

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