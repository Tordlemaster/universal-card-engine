use crate::rules::{deck::DeckVisibility, game::GameWorld, routine::evaluatables::{DeckVisibilityEvaluatable, EvaluatableString, VarBindSetEvaluatable}, state::StateSwitchData, variable::VarBindSet};

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
    fn execute (&self, bindings: &VarBindSet, game_world: &mut crate::rules::game::GameWorld) -> Option<StateSwitchData> {
        let name = self.name.evaluate(&bindings, game_world);
        let visibility = self.visibility.evaluate(&bindings, game_world);
        game_world.add_deck(name, visibility);

        None
    }
    fn undo (&self, bindings: &VarBindSet, game_world: &mut GameWorld) -> () {
        let name = self.name.evaluate(&bindings, game_world);
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
    fn execute (&self, bindings: &VarBindSet, game_world: &mut crate::rules::game::GameWorld) -> Option<StateSwitchData> {
        let name = self.name.evaluate(&bindings, game_world);
        let visibility = self.visibility.evaluate(&bindings, game_world);
        game_world.add_source_deck(name, visibility);

        None
    }
    fn undo (&self, bindings: &VarBindSet, game_world: &mut GameWorld) -> () {
        let name = self.name.evaluate(&bindings, game_world);
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
    fn execute (&self, bindings: &VarBindSet, game_world: &mut GameWorld) -> Option<StateSwitchData> {
        let name = self.name.evaluate(bindings, game_world);
        if let Some(deck) = game_world.get_deck(&name) {
            //self.visibility = Some(deck.visibility().clone());
        }
        game_world.remove_deck(&self.name.evaluate(bindings, game_world));
        //TODO make sure that if self.visibility is not None when trying to 
        //TODO TODO self.visibility and all instances of Routines storing internal state is dangerous
        //because it will be hard to maintain the internal state over game loops
        None
    }

    fn undo (&self, bindings: &VarBindSet, game_world: &mut GameWorld) -> () {
        //TODO
    }
}

pub struct DealRoutine {
    source: EvaluatableString,
    dest: EvaluatableString,
    n: usize
}

impl DealRoutine {
    pub fn new(source: &String, dest: &String, n: usize) -> DealRoutine{
        DealRoutine { source: EvaluatableString::new(source), dest: EvaluatableString::new(dest), n: n }
    }
}

impl Routine for DealRoutine {
    fn execute (&self, bindings: &VarBindSet, game_world: &mut GameWorld) -> Option<StateSwitchData> {
        if let Err(n_dealt) = game_world.deal(&self.source.evaluate(bindings, game_world), &self.dest.evaluate(bindings, game_world), self.n) {
            //TODO replace panic with a real action
            panic!("Tried to draw from an empty deck {}!", self.source.evaluate(bindings, game_world));
        }
        None
    }
    fn undo (&self, bindings: &VarBindSet, game_world: &mut GameWorld) -> () {
        if let Err(n_dealt) = game_world.deal(&self.source.evaluate(bindings, game_world), &self.dest.evaluate(bindings, game_world), self.n) {
            //TODO replace panic with a real action
            panic!("PROGRAM ERROR: Tried to undo a deal action and drew from an empty deck {}, meaning execute() did not deal all or any cards first!", self.source.evaluate(bindings, game_world));
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
    fn execute (&self, bindings: &VarBindSet, game_world: &mut GameWorld) -> Option<StateSwitchData> {
        Some(StateSwitchData::new(self.name.clone(), self.bindings.evaluate(bindings, game_world)))
    }
    fn undo (&self, bindings: &VarBindSet, game_world: &mut GameWorld) -> () {
        panic!("Script error: tried to undo a state switch, which is not allowed");
    }
}

pub struct NullRoutine;

impl NullRoutine {
    pub fn new() -> NullRoutine {
        NullRoutine{}
    }
}

impl Routine for NullRoutine {
    fn execute (&self, bindings: &VarBindSet, game_world: &mut GameWorld) -> Option<StateSwitchData> {
        None
    }
    fn undo (&self, bindings: &VarBindSet, game_world: &mut GameWorld) -> () {
        
    }
}