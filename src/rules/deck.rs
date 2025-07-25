use std::collections::{hash_map::Iter, HashMap};

use rand::random_range;

pub struct CardAttr {
    pub name: String,
    pub abbrv: String,
    pub val: u32
}

impl CardAttr {
    pub fn new(name: String, abbrv: String, val: u32) -> CardAttr {
        CardAttr { name: name, abbrv: abbrv, val: val}
    }
}

pub struct CardSetData {
    pub suits: Vec<CardAttr>,
    pub values: Vec<CardAttr>,

    ///How many cards per source deck
    pub cards_per_deck: usize,

    ///How many decks' worth of cards are being used in this game
    pub num_decks: usize
}

impl CardSetData {
    pub fn new(suit_names: Vec<CardAttr>, value_names: Vec<CardAttr>, decks: usize) -> CardSetData {
        CardSetData {
            cards_per_deck: suit_names.len() * value_names.len(),
            suits: suit_names,
            values: value_names,
            num_decks: decks
        }
    }
    pub fn total_cards(&self) -> usize {
        self.num_decks * self.cards_per_deck
    }
}

pub struct Card {
    data: u8
}

impl Card {
    pub fn get_suit(&self) -> usize {
        (self.data & 0b00000011) as usize
    }
    pub fn get_value(&self) -> usize {
        (self.data >> 2) as usize
    }
}

//Use a HeldCard struct to contain the card in the hand and also store information about whether the card is turned
#[derive(Clone)]
pub struct DeckVisibility {
    /// When true, the cards are stacked and only the top one is visible to players with permission.
    /// When false, the cards are spread and all are visible to players with permission.
    stack: bool,

    visible_to_all: bool,

    ///Names of players who can see the contents of this deck
    players_visible: Vec<String>,

    //Numbers of teams who can see the contents of this deck
    teams_visible: Vec<usize>
}

impl DeckVisibility {
    pub fn new(stack: bool, visible_to_all: bool, players_visible: Vec<String>, teams_visible: Vec<usize>) -> DeckVisibility {
        DeckVisibility { stack: stack, visible_to_all: visible_to_all, players_visible: players_visible, teams_visible: teams_visible }
    }

    ///Visible to all players
    pub fn visible_all() {

    }
}

pub struct Deck {
    cards: Vec<Card>,

    visibility: DeckVisibility
}

impl Deck {
    pub fn new(cards: Vec<Card>, visibility: DeckVisibility) -> Deck {
        Deck { cards: cards, visibility: visibility }
    }


    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
    }
    pub fn visibility(&self) -> &DeckVisibility {
        &self.visibility
    }
    pub fn stack(&self) -> bool {
        self.visibility.stack
    }
    pub fn visible_to_all(&self) -> bool {
        self.visibility.visible_to_all
    }
    pub fn players_visible(&self) -> &Vec<String> {
        &self.visibility.players_visible
    }
    pub fn teams_visible(&self) -> &Vec<usize> {
        &self.visibility.teams_visible
    }

    ///Draw a random card
    pub fn draw_card(&mut self) -> Option<(usize, Card)> {
        if self.cards.len() > 0 {
            let i = random_range(0..self.cards.len());
            Some((i, self.cards.remove(i)))
        }
        else {
            None
        }
    }


    ///Draw card at index idx
    pub fn draw_card_idx(&mut self, idx: usize) -> Option<Card> {
        if self.cards.len() > 0 {
            Some(self.cards.remove(idx))
        }
        else {
            None
        }
    }
    pub fn draw_card_top(&mut self) -> Option<Card> {
        self.cards.pop()
    }
    pub fn insert_card(&mut self, card: Card) {
        self.cards.push(card);
    }
    pub fn card_iter(&self) -> std::slice::Iter<Card> {
        self.cards.iter()
    }
    pub fn len(&self) -> usize {
        self.cards.len()
    }
}

pub struct DeckSet {
    decks: HashMap<String, Deck>
}

impl DeckSet {
    pub fn new() -> DeckSet {
        DeckSet { decks: HashMap::new() }
    }

    pub fn add_source_deck(&mut self, card_quantity: usize, name: String, visibility: DeckVisibility) {
        self.decks.insert(name, Deck::new((0..card_quantity).map(|x| Card{data: x as u8}).collect(), visibility));
    }
    pub fn add_deck(&mut self, name: String, visibility: DeckVisibility) {
        self.decks.insert(name, Deck::new(Vec::new(), visibility));
    }
    pub fn remove_deck(&mut self, name: &String) {
        if let Some(d) = self.decks.get(name) {
            if d.len() == 0 {
                self.decks.remove(name);
            }
            else {
                panic!("Script error: tried to remove non-empty deck {}", name);
            }
        }
        else {
            panic!("Script error: tried to remove non-existent deck {}", name);
        }
    }

    pub fn iter_decks(&self) -> Iter<String, Deck> {
        self.decks.iter()
    }

    pub fn get_deck(&self, name: &String) -> Option<&Deck> {
        self.decks.get(name)
    }
    pub fn get_deck_mut(&mut self, name: &String) -> Option<&mut Deck> {
        self.decks.get_mut(name)
    }
}