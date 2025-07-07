pub struct Rule <'a> {
    nonterminal: &'a str,
    production: &'a [&'a str]
}

const grammar: [Rule; 3] = [
    Rule {nonterminal: &"_root", production: &["_states_def"]},
    Rule {nonterminal: &"_states_def", production: &["STATES", "_states_body"]},
    Rule {nonterminal: &"_states_body", production: &["_state_def", "_states_body"]}
];