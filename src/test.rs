use lalrpop_util::lalrpop_mod;

use crate::rules::{deck::{CardAttr, CardSetData, DeckSet}, game::{Game, GameWorld}, player::PlayerSet};

lalrpop_mod!(grammar);

pub fn test_() {
    let csd = CardSetData::new(
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
    let p = grammar::StatesParser::new();
    match p.parse(r#"STATES { "SETUP" { PRINT "Hell#o'" STATE "_END" {} } }"#) {
        Ok(ss) => {
            Game::new(vec![], csd, ss).launch();
        }
        Err(e) => {println!("{}", e);}
    }
    println!("{}", grammar::VarBindListParser::new().parse("{}").is_ok());
}