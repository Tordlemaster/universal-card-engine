use crate::rules::{deck::DeckVisibility, game::GameWorld, variable::VarBindSet};

pub struct EvaluatableString {
    base: String,
    pub var_first: bool,
    pub non_var_slices: Vec<String>,
    pub var_slices: Vec<String>
}

impl EvaluatableString {
    pub fn new(s: &String) -> EvaluatableString {
        let mut es: EvaluatableString = EvaluatableString{base: s.clone(), non_var_slices: Vec::new(), var_slices: Vec::new(), var_first: false};
        
        //Whether the index is within a variable surrounded by square brackets
        let mut in_sq_br = false;
        let mut start: usize = 0;
        for i in 0..s.as_bytes().len() {
            if in_sq_br {
                if s.as_bytes()[i] == b']' {
                    //Variable ends
                    let var = es.base[start..i].to_string();
                    //if var.as_bytes()[0_usize] == b'#' {
                    //    es.pound_mode = true;
                    //}
                    es.var_slices.push(var);
                    start = i + 1;
                    in_sq_br = false;
                }
                else {
                    //Variable continues
                    if s.as_bytes()[i] == b'[' {
                        panic!("Script error: invalid variable syntax \"{}\"", s);
                    }
                }
            }
            else {
                if s.as_bytes()[i] == b'[' {
                    //Variable starts
                    if i==0 {
                        es.var_first = true;
                    }
                    es.non_var_slices.push(es.base[start..i].to_string());
                    start = i + 1;
                    in_sq_br = true;
                }
                else {
                    //Non-variable continues
                    if s.as_bytes()[i] == b']' {
                        panic!("Script error: invalid variable syntax \"{}\"", s);
                    }
                }
            }
        }

        //Only non-vars go to the end without a square bracket marking their end
        es.non_var_slices.push(es.base[start..].to_string());

        es
    }
    pub fn evaluate(&self, bindings: &VarBindSet, game_world: &GameWorld) -> String {
        let mut n = 0;
        let mut s_start = String::new();
        let mut s_end = String::new();
        let mut s_pound = String::new();
        let mut s_active = &mut s_start;
        let mut has_seen_pound = false;

        let mut use_var = false;//self.var_first;

        for i in 0..(self.var_slices.len() + self.non_var_slices.len()) {
            if use_var {
                let v = &self.var_slices[i/2];
                if v.as_bytes()[0] == b'#' {
                    if has_seen_pound {
                        panic!("Script error: no more than one variable beginning with '#' is permitted in each string")
                    }
                    has_seen_pound = true;
                    s_active = &mut s_end;
                    s_pound.push_str(v);
                }
                else {
                //println!("Pushing var {}: {}", i/2, self.non_var_slices[i/2].as_str());
                    s_active.push_str(bindings.get_str_val(v).expect("Script error: variable not found").as_str());
                }
            }
            else {
                //println!("Pushing non-var {}: {}", i/2, self.non_var_slices[i/2].as_str());
                s_active.push_str(self.non_var_slices[i/2].as_str());
            }
            use_var = !use_var;
        }

        if has_seen_pound {
            if let Some(pound_value) = bindings.get_str_val(&s_pound) {
                s_start.push_str(&pound_value);
                s_start.push_str(&s_end);
            }
            else {
                //Pattern-match on the deck names and increase the value by one
                s_start.push_str(&EvaluatableString::pattern_match(s_start.as_str(), s_end.as_str(), bindings, game_world).to_string());
                s_start.push_str(&s_end);
            }
        }

        s_start
    }
    ///Returns 1 plus the value found by pattern matching deck names for the pound variable's position
    fn pattern_match(start: &str, end: &str, bindings: &VarBindSet, game_world: &GameWorld) -> usize {
        for (deck_name, _) in game_world.get_decks() {
            if deck_name.len() > start.len() + end.len() {
                if deck_name.starts_with(start) && deck_name.ends_with(end) {
                    let val = &deck_name[start.len()..deck_name.len()-end.len()];
                    let new_val = (val.parse::<usize>().expect("# variable not a usize") + 1);

                    return new_val;
                }
            }
        }
        //We are creating the first name that follows the pattern
        1
    }
}

pub struct EvaluatableStringUInt {
    e: EvaluatableString
}

impl EvaluatableStringUInt {
    pub fn new(s: &String) -> EvaluatableStringUInt {
        EvaluatableStringUInt { e: EvaluatableString::new(s) }
    }
}

pub struct DeckVisibilityEvaluatable {
    stack: bool,
    visible_to_all: bool,
    players_visible: Vec<EvaluatableString>,
    teams_visible: Vec<EvaluatableString>
}

impl DeckVisibilityEvaluatable {
    pub fn new(stack: bool, visible_to_all: bool, players_visible: Vec<String>, teams_visible: Vec<usize>) -> DeckVisibilityEvaluatable {
        DeckVisibilityEvaluatable {
            stack: stack,
            visible_to_all: visible_to_all,
            players_visible: players_visible.iter().map(|x| EvaluatableString::new(x)).collect(),
            teams_visible: Vec::new()//teams_visible.iter().map(|x| EvaluatableString::new(x)).collect()
        }
    }
    pub fn evaluate(&self, bindings: &VarBindSet, game_world: &GameWorld) -> DeckVisibility {
        DeckVisibility::new(
            self.stack,
            self.visible_to_all,
            self.players_visible.iter().map(|e| e.evaluate(bindings, game_world)).collect(),
            Vec::new() //TODO self.teams_visible.iter().map(|e| e.evaluate(bindings, game_world)).collect()
        )
    }
}

pub struct VarBindSetEvaluatable {
    vars: Vec<(String, EvaluatableString)>
}

impl VarBindSetEvaluatable {
    pub fn new(vars: Vec<(String, String)>) -> VarBindSetEvaluatable {
        VarBindSetEvaluatable { vars: vars.iter().map(|(n, v)| (n.clone(), EvaluatableString::new(v))).collect() }
    }
    pub fn evaluate(&self, bindings: &VarBindSet, game_world: &GameWorld) -> VarBindSet {
        let mut new_bindings = VarBindSet::new();

        for (name, value) in &self.vars {
            new_bindings.insert_str_var(name, value.evaluate(bindings, game_world));
        }

        new_bindings
    }
}