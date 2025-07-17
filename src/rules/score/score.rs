use crate::rules::{deck::{CardSetData, Deck}, routine::{evaluatables::EvaluatableString, routine::Routine}};

fn score_total_card_vals(deck: &Deck, csd: &CardSetData) -> u32 {
    let mut score: u32 = 0;

    for c in deck.card_iter() {
        score += csd.values[c.get_value()].val;
    }

    score
}

///Evaluates a deck with a scoring function and adds the score to a player
pub struct ScoreAddRoutine {
    func: fn (&Deck, &CardSetData) -> u32,
    deck_name: EvaluatableString,
    player_name: EvaluatableString,
    //prev_binding: Option<(String, u32)>,
    prev_score_inc: Option<u32>
}

impl ScoreAddRoutine {
    pub fn new(deck_name: &String, player_name: &String) -> ScoreAddRoutine {
        ScoreAddRoutine { func: score_total_card_vals, deck_name: EvaluatableString::new(deck_name), player_name: EvaluatableString::new(player_name), prev_score_inc: None }
    }
}

impl Routine for ScoreAddRoutine {
    fn execute (&mut self, bindings: &crate::rules::variable::VarBindSet, game_world: &mut crate::rules::game::GameWorld, choice_vars: &mut crate::rules::variable::TempVars) -> Option<crate::rules::state::StateSwitchData> {
        let s = (self.func)(game_world.get_deck(&self.deck_name.evaluate(bindings, game_world, choice_vars)).unwrap(), game_world.get_card_set_data());
        game_world.add_player_score(&self.player_name.evaluate(bindings, game_world, choice_vars), s);
        None
    }
    fn undo (&mut self, bindings: &crate::rules::variable::VarBindSet, game_world: &mut crate::rules::game::GameWorld, choice_vars: &mut crate::rules::variable::TempVars) -> () {
        
    }
}