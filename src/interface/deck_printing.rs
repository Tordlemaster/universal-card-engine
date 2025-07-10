use std::fmt::format;

use crate::rules::{deck::{Card, CardSetData, Deck}, game::GameWorld, player::Player};

pub fn print_all_decks(game_world: &GameWorld, player: &Player) {
    println!();
    for (deck_name, deck) in game_world.get_decks() {
        print_deck(deck_name, deck, player, game_world.get_card_set_data());
        println!();
    }
}

pub fn print_deck(deck_name: &String, deck: &Deck, player: &Player, card_set_data: &CardSetData) {
    //Name of the deck
    println!("{}:", deck_name);

    if deck.len() > 0 {
        if deck.visible_to_all() || deck.players_visible().contains(player.name()) || deck.teams_visible().contains(player.team()) {
            match deck.stack() {
                true => {
                    //"Card on top name" + N more underneath
                    print!("{}", card_to_str(&deck.cards()[deck.len()-1], card_set_data));
                    if deck.len() > 1 {
                        println!(" + {} more underneath", deck.len() - 1);
                    }
                }
                false => {
                    //Card names separated by spaces
                    for card in deck.cards() {
                        print!("{}  ", card_to_str(card, card_set_data));
                    }
                    println!("");
                }
            }
        }
        else {
            println!("{} unknown cards", deck.len());
        }
    }
    else {
        println!("Empty");
    }
}

pub fn card_to_str(card: &Card, card_set_data: &CardSetData) -> String {
    format!("{} of {}", card_set_data.value_names[card.get_value()].name, card_set_data.suit_names[card.get_suit()].name)
}