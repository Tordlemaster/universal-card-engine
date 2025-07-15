use std::fs;

use lalrpop_util::lalrpop_mod;

use crate::{interface::deck_printing::{print_all_decks, print_deck}, rules::{deck::{CardAttr, CardSetData, DeckVisibility}, game::{Game, GameWorld}, player::Player, routine::evaluatables::EvaluatableString, state::StateSet, variable::VarBindSet}};

pub mod rules;
pub mod interface;
pub mod script;

mod test_rummy;
mod test;

lalrpop_mod!(grammar);

fn test() {
    /*let players = vec![Player::new("bip".to_string(), 0), Player::new("bop".to_string(), 1)];

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
    let mut world = Game::new(players, card_set_data, StateSet::new(Vec::new(), Vec::new()));

    world.add_source_deck(
        "Draw pile".to_string(),
        DeckVisibility::new(
            true, false, world.get_players().names().clone(), world.get_players().teams().clone()
        )
    );

    print_all_decks(&world.world(), &world.get_players().get_player_by_idx(0).unwrap());

    world.add_deck(
        "bip's hand".to_string(),
        DeckVisibility::new(
            false, false, vec!["bip".to_string()], vec![0]
        )
    );

    world.deal(&"Draw pile".to_string(), &"bip's hand".to_string(), 3);

    print_all_decks(&world.world(), &world.get_players().get_player_by_idx(0).unwrap());

    print_all_decks(&world.world(), &world.get_players().get_player_by_idx(0).unwrap());

    let mut bindings = VarBindSet::new();
    bindings.insert_str_var(&String::from("b"), String::from("bip"));
    println!("\n{}", &bindings.get_str_val(&"b".to_string()).unwrap());
    let eval = EvaluatableString::new(&String::from("[b]'s hand"));
    println!("{} {:?} {:?}", eval.var_first, eval.non_var_slices, eval.var_slices);
    println!("\ne: {}\n", eval.evaluate(&bindings, &world.world()));

    let b = eval.evaluate(&bindings, &world.world());
    print_deck(&b, world.get_deck(&b).unwrap(), &world.get_players().get_player_by_idx(0).unwrap(), world.get_card_set_data());

    let pound1 = EvaluatableString::new(&String::from("Meld [#]"));
    world.add_deck(pound1.evaluate(&bindings, &world.world()), DeckVisibility::new(false, true, Vec::new(), Vec::new()));

    print_all_decks(&world.world(), &world.get_players().get_player_by_idx(0).unwrap());

    world.add_deck(pound1.evaluate(&bindings, &world.world()), DeckVisibility::new(false, true, Vec::new(), Vec::new()));

    print_all_decks(&world.world(), &world.get_players().get_player_by_idx(0).unwrap());*/
}

fn parse_script(path: String) -> Result<Game, ()> {
    if let Ok(script) = fs::read_to_string(path) {
        let p = grammar::StatesParser::new();
        match p.parse(&script.as_str()) {
            Ok(ss) => {
                Ok(Game::new(
                    vec![Player::new("bip".to_string(), 0), Player::new("bop".to_string(), 1)],
                    CardSetData::new(
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
                    ),
                    ss
                ))
            }
            Err(e) => {
                println!("{}", e);
                Err(())
            }
        }
    }
    else {
        println!("Invalid filepath");
        Err(())
    }
}

fn main() {
    if true {
        //let mut g = test_rummy::rummy();
        if let Ok(mut g) = parse_script("./games/rummy.uce".to_string()) {
            g.launch();
            print_all_decks(&g.world(), &g.world().get_players().get_player_by_idx(0).unwrap());
        }
        else {panic!()}
    }
    else {
        //println!("{}", grammar::PlayerNameListParser::new().parse("[ ]").is_ok())
        test::test_();
    }
}