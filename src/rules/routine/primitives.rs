use crate::{interface::{deck_printing::print_all_decks, interface::card_subset_interface}, rules::{deck::DeckVisibility, game::GameWorld, routine::{choice_routine::ChoiceLimit, evaluatables::{DeckVisibilityEvaluatable, EvaluatableString, VarBindSetEvaluatable}}, state::StateSwitchData, variable::{TempVars, VarBindSet}}};

use super::routine::*;

pub struct CreateDeckRoutine {
    name: EvaluatableString,
    visibility: DeckVisibilityEvaluatable
}

impl CreateDeckRoutine {
    pub fn new(deck_name: &String, visibility: DeckVisibilityEvaluatable) -> CreateDeckRoutine {
        CreateDeckRoutine { name: EvaluatableString::new(deck_name), visibility: visibility }
    }
}

impl Routine for CreateDeckRoutine {
    fn execute (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> Option<StateSwitchData> {
        let name = self.name.evaluate_create(&bindings, game_world, choice_vars);
        //println!("Creating deck '{}'", &name);
        let visibility = self.visibility.evaluate(&bindings, game_world, choice_vars);
        game_world.add_deck(name, visibility);

        None
    }
    fn undo (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> () {
        let name = self.name.evaluate(&bindings, game_world, choice_vars);
        //println!("Removing deck '{}'", &name);
        game_world.remove_deck(&name);
    }
}

pub struct CreateSourceDeckRoutine {
    name: EvaluatableString,
    visibility: DeckVisibilityEvaluatable
}

impl CreateSourceDeckRoutine {
    pub fn new(deck_name: &String, visibility: DeckVisibilityEvaluatable) -> CreateSourceDeckRoutine {
        CreateSourceDeckRoutine { name: EvaluatableString::new(deck_name), visibility: visibility }
    }
}

impl Routine for CreateSourceDeckRoutine {
    fn execute (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> Option<StateSwitchData> {
        let name = self.name.evaluate_create(&bindings, game_world, choice_vars);
        let visibility = self.visibility.evaluate(&bindings, game_world, choice_vars);
        game_world.add_source_deck(name, visibility);

        None
    }
    fn undo (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> () {
        let name = self.name.evaluate(&bindings, game_world, choice_vars);
        game_world.remove_deck(&name);
    }
}

pub struct RemoveDeckRoutine {
    name: EvaluatableString,
    visibility: Option<DeckVisibility>
}

impl RemoveDeckRoutine {
    pub fn new(name: String) -> RemoveDeckRoutine {
        RemoveDeckRoutine { name: EvaluatableString::new(&name), visibility: None }
    }
}

impl Routine for RemoveDeckRoutine {
    fn execute (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> Option<StateSwitchData> {
        let name = self.name.evaluate(bindings, game_world, choice_vars);
        if let Some(deck) = game_world.get_deck(&name) {
            //self.visibility = Some(deck.visibility().clone());
        }
        game_world.remove_deck(&self.name.evaluate(bindings, game_world, choice_vars));
        //TODO make sure that if self.visibility is not None when trying to 
        //TODO TODO self.visibility and all instances of Routines storing internal state is dangerous
        //because it will be hard to maintain the internal state over game loops
        None
    }

    fn undo (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> () {
        //TODO
    }
}

///For dealing n random cards from source to the end of dest
pub struct DealRandRoutine {
    source: EvaluatableString,
    dest: EvaluatableString,
    n: usize
}

impl DealRandRoutine {
    pub fn new(source: &String, dest: &String, n: usize) -> DealRandRoutine{
        DealRandRoutine { source: EvaluatableString::new(source), dest: EvaluatableString::new(dest), n: n }
    }
}

impl Routine for DealRandRoutine {
    fn execute (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> Option<StateSwitchData> {
        if let Err(n_dealt) = game_world.deal(&self.source.evaluate(bindings, game_world, choice_vars), &self.dest.evaluate(bindings, game_world, choice_vars), self.n) {
            //TODO replace panic with a real action
            panic!("Tried to draw from an empty deck {}!", self.source.evaluate(bindings, game_world, choice_vars));
        }
        None
    }
    fn undo (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> () {
        if let Err(n_dealt) = game_world.deal(&self.source.evaluate(bindings, game_world, choice_vars), &self.dest.evaluate(bindings, game_world, choice_vars), self.n) {
            //TODO replace panic with a real action
            panic!("PROGRAM ERROR: Tried to undo a deal action and drew from an empty deck {}, meaning execute() did not deal all or any cards first!", self.source.evaluate(bindings, game_world, choice_vars));
        }
    }
}

pub struct DealTopRoutine {
    source: EvaluatableString,
    dest: EvaluatableString,
    n: usize
}

impl DealTopRoutine {
    pub fn new(source: &String, dest: &String, n: usize) -> DealTopRoutine{
        DealTopRoutine { source: EvaluatableString::new(source), dest: EvaluatableString::new(dest), n: n }
    }
}

impl Routine for DealTopRoutine {
    fn execute (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> Option<StateSwitchData> {
        if let Err(n_dealt) = game_world.deal_top(&self.source.evaluate(bindings, game_world, choice_vars), &self.dest.evaluate(bindings, game_world, choice_vars), self.n) {
            //TODO replace panic with a real action
            panic!("Tried to draw from an empty deck {}!", self.source.evaluate(bindings, game_world, choice_vars));
        }
        None
    }
    fn undo (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> () {
        if let Err(n_dealt) = game_world.deal_top(&self.source.evaluate(bindings, game_world, choice_vars), &self.dest.evaluate(bindings, game_world, choice_vars), self.n) {
            //TODO replace panic with a real action
            panic!("PROGRAM ERROR: Tried to undo a deal action and drew from an empty deck {}, meaning execute() did not deal all or any cards first!", self.source.evaluate(bindings, game_world, choice_vars));
        }
    }
}


///For dealing n cards from specific indices in source to the end of dest
pub struct DealSpecificRoutine {
    source: EvaluatableString,
    dest: EvaluatableString,
    n: ChoiceLimit,
    ///The indices of the cards dealt in the last call of execute(), for use in undo()
    exec_idcs: Vec<usize>
}

impl DealSpecificRoutine {
    pub fn new(source: &String, dest: &String, n: ChoiceLimit) -> DealSpecificRoutine{
        DealSpecificRoutine { source: EvaluatableString::new(source), dest: EvaluatableString::new(dest), n: n, exec_idcs: Vec::new() }
    }
}

impl Routine for DealSpecificRoutine {
    fn execute (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> Option<StateSwitchData> {
        //TODO TODO TODO TODO
        //println!("Executing DealSpecificRoutine");
        let source_name = self.source.evaluate(bindings, game_world, choice_vars);
        let source_deck = game_world.get_deck(&source_name).expect("Script error: deck not found");
        let v = card_subset_interface(source_deck, &source_name, self.n, game_world.get_card_set_data());
        
        ///The indices of each card pulled from source so far at the moment it was pulled, so taking offset into account
        let mut dealt_offset_indices = Vec::new();

        for i in 0..(v.len()) {
            let idx = v[i];
            let offset: usize = v[0..i].iter().map(|x| (*x < idx) as usize).sum();
            dealt_offset_indices.push(idx - offset);
            
            if let Err(n_dealt) = game_world.deal_idx(&self.source.evaluate(bindings, game_world, choice_vars), &self.dest.evaluate(bindings, game_world, choice_vars), idx - offset) {
                //TODO replace panic with a real action
                self.exec_idcs = dealt_offset_indices;
                panic!("Tried to draw from an empty deck {}!", source_name);
            }
        }

        self.exec_idcs = dealt_offset_indices;
        
        None
    }
    fn undo (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> () {
        //println!("Undoing DealSpecificRoutine");
        let dest_name = self.dest.evaluate(bindings, game_world, choice_vars);
        let dest_deck = game_world.get_deck(&dest_name).expect(format!("Script error: deck {} not found", dest_name).as_str());
        let mut l = dest_deck.len();
        for idx in self.exec_idcs.iter().rev() {
            l -= 1;
            if let Err(n_dealt) = game_world.deal_idx(&self.dest.evaluate(bindings, game_world, choice_vars), &self.source.evaluate(bindings, game_world, choice_vars), l) {
                //TODO replace panic with a real action
                panic!("undo() for DealSpecificRoutine failed");
            }
        }
    }
}



pub struct StateSwitchRoutine {
    ///Name of the state to switch to
    name: String,
    
    ///Will evaluate to the bindings passed as part of StateSwitchData to be added to the next state
    bindings: VarBindSetEvaluatable
}

impl StateSwitchRoutine {
    pub fn new(name: String, bindings: Vec<(String, String)>) -> StateSwitchRoutine {
        StateSwitchRoutine { name: name, bindings: VarBindSetEvaluatable::new(bindings) }
    }
}

impl Routine for StateSwitchRoutine {
    fn execute (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> Option<StateSwitchData> {
        Some(StateSwitchData::new(self.name.clone(), self.bindings.evaluate(bindings, game_world, choice_vars)))
    }
    fn undo (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> () {
        panic!("Script error: tried to undo a state switch, which is not allowed");
    }
}

pub struct PrintDecksRoutine {
    var: EvaluatableString
}

impl PrintDecksRoutine {
    pub fn new() -> PrintDecksRoutine {
        PrintDecksRoutine { var: EvaluatableString::new(&String::from("[THISPLAYER]")) }
    }
}

impl Routine for PrintDecksRoutine {
    fn execute (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> Option<StateSwitchData> {
        print_all_decks(game_world, &game_world.get_players().get_player_by_name(&self.var.evaluate(bindings, game_world, choice_vars)).unwrap());
        None
    }
    fn undo (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> () {}
}

pub struct LoopRoutine {
    routine: Box<dyn Routine>
}

impl LoopRoutine {
    pub fn new(routine: Box<dyn Routine>) -> LoopRoutine {
        LoopRoutine { routine: routine }
    }
}

impl Routine for LoopRoutine {
    fn execute (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> Option<StateSwitchData> {
        loop {
            let ret = self.routine.execute(bindings, game_world, choice_vars);
            
            if ret.is_some() {
                return ret;
            }
        }
    }
    fn undo (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> () {
        
    }
}

pub struct PrintMsgRoutine {
    msg: EvaluatableString
}

impl PrintMsgRoutine {
    pub fn new(s: &String) -> PrintMsgRoutine {
        PrintMsgRoutine { msg: EvaluatableString::new(s) }
    }
}

impl Routine for PrintMsgRoutine {
    fn execute (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> Option<StateSwitchData> {
        println!("{}", &self.msg.evaluate(bindings, game_world, choice_vars));
        None
    }
    fn undo (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> () {
        
    }
}

pub struct NullRoutine;

impl NullRoutine {
    pub fn new() -> NullRoutine {
        NullRoutine{}
    }
}

impl Routine for NullRoutine {
    fn execute (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> Option<StateSwitchData> {
        None
    }
    fn undo (&mut self, bindings: &VarBindSet, game_world: &mut GameWorld, choice_vars: &mut TempVars) -> () {
        
    }
}