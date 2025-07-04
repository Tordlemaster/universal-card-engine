use crate::{interface::deck_printing::print_all_decks, rules::{deck::{CardAttr, CardSetData, DeckVisibility}, game::GameWorld, player::Player}};

pub mod rules;
pub mod interface;

fn main() {
    let players = vec![Player::new("bip".to_string(), 0), Player::new("bop".to_string(), 1)];

    let card_set_data = CardSetData::new(
        vec![
            CardAttr::new("Clubs".to_string(), "C".to_string()),
            CardAttr::new("Spades".to_string(), "S".to_string()),
            CardAttr::new("Hearts".to_string(), "H".to_string()),
            CardAttr::new("Diamonds".to_string(), "D".to_string())
        ],
        vec![
            CardAttr::new("Ace".to_string(), "A".to_string()),
            CardAttr::new("Two".to_string(), "2".to_string()),
            CardAttr::new("Three".to_string(), "3".to_string()),
            CardAttr::new("Four".to_string(), "4".to_string()),
            CardAttr::new("Five".to_string(), "5".to_string()),
            CardAttr::new("Six".to_string(), "6".to_string()),
            CardAttr::new("Seven".to_string(), "7".to_string()),
            CardAttr::new("Eight".to_string(), "8".to_string()),
            CardAttr::new("Nine".to_string(), "9".to_string()),
            CardAttr::new("Jack".to_string(), "J".to_string()),
            CardAttr::new("Queen".to_string(), "Q".to_string()),
            CardAttr::new("King".to_string(), "K".to_string()),
        ],
        1
    );
    let mut world = GameWorld::new(players, card_set_data);

    world.add_source_deck(
        "Draw pile".to_string(),
        DeckVisibility::new(
            true, world.get_players().names().clone(), world.get_players().teams().clone()
        )
    );

    print_all_decks(&world, &world.get_players().get_player(0));

    world.add_deck(
        "bip's hand".to_string(),
        DeckVisibility::new(
            false, vec!["bip".to_string()], vec![0]
        )
    );

    world.deal(&"Draw pile".to_string(), &"bip's hand".to_string(), 3);

    print_all_decks(&world, &world.get_players().get_player(0));

    print_all_decks(&world, &world.get_players().get_player(1));
}
