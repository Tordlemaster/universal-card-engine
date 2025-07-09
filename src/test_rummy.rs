use crate::rules::{conditional::{conditional::{ConditionalMode, MultiConditional, TrueConditional}, deck_conditional::{DeckConditional, DeckSuitsComp, DeckSuitsConditional, DeckValsComp, DeckValsConditional}}, deck::{CardAttr, CardSetData}, game::{Game, GameWorld}, player::Player, routine::{choice_routine::{Choice, ChoiceLimit, ChoicesRoutine}, cond_routine::{CondRoutine, CondRoutineMode}, evaluatables::{DeckVisibilityEvaluatable, EvaluatableString}, iter_routine::ForPlayerRoutine, primitives::{CreateDeckRoutine, CreateSourceDeckRoutine, DealRandRoutine, DealSpecificRoutine, LoopRoutine, PrintDecksRoutine, StateSwitchRoutine}, routine::SeqRoutine}, state::{State, StateSet}, variable::VarBindSet};

pub fn rummy() -> Game {
    let game = Game::new(
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
        StateSet::new(
            vec!["SETUP".to_string(), "MAIN".to_string()],// "SCORING".to_string()],
            vec![
                Some(State::new(
                    Box::new(SeqRoutine::new(vec![
                        Box::new(CreateSourceDeckRoutine::new(&"Draw pile".to_string(), DeckVisibilityEvaluatable::new(true, false, Vec::new(), Vec::new()))),
                        Box::new(CreateDeckRoutine::new(&"Discard pile".to_string(), DeckVisibilityEvaluatable::new(true, true, Vec::new(), Vec::new()))),
                        Box::new(ForPlayerRoutine::new(
                            Box::new(SeqRoutine::new(vec![
                                Box::new(CreateSourceDeckRoutine::new(&"[THISPLAYER]'s hand".to_string(), DeckVisibilityEvaluatable::new(false, false, vec!["[THISPLAYER]".to_string()], Vec::new()))),
                                Box::new(DealRandRoutine::new(&"Draw pile".to_string(), &"[THISPLAYER]'s hand".to_string(), 10))
                            ]))
                        )),
                        Box::new(DealRandRoutine::new(&"Draw pile".to_string(), &"Discard pile".to_string(), 1)),
                        Box::new(StateSwitchRoutine::new("MAIN".to_string(), Vec::new()))
                    ]))
                )),
                Some(State::new(
                    Box::new(SeqRoutine::new(vec![
                        Box::new(LoopRoutine::new(
                            Box::new(ForPlayerRoutine::new(
                                Box::new(SeqRoutine::new(vec![
                                    Box::new(PrintDecksRoutine::new()),
                                    Box::new(ChoicesRoutine::new(
                                        vec![
                                            Choice::new(
                                                "Draw from draw pile".to_string(),
                                                CondRoutine::without_cond(
                                                    Box::new(DealRandRoutine::new(&"Draw pile".to_string(), &"[THISPLAYER]'s hand".to_string(), 1)),
                                                )
                                            ),
                                            Choice::new(
                                                "Draw from discard pile".to_string(),
                                                CondRoutine::without_cond(
                                                    Box::new(DealRandRoutine::new(&"Discard pile".to_string(), &"[THISPLAYER]'s hand".to_string(), 1)),
                                                )
                                            )
                                        ],
                                        ChoiceLimit::Limited(1)
                                    )),
                                    Box::new(PrintDecksRoutine::new()),
                                    Box::new(ChoicesRoutine::new(
                                        vec![
                                            Choice::new(
                                                "Meld".to_string(),
                                                CondRoutine::new(
                                                    Box::new(MultiConditional::new(
                                                        vec![
                                                            Box::new(DeckConditional::new(
                                                                vec![
                                                                    Box::new(DeckSuitsConditional::new(DeckSuitsComp::Same)),
                                                                    Box::new(DeckValsConditional::new(DeckValsComp::Consecutive))
                                                                ],
                                                                ConditionalMode::And,
                                                                &"Meld [#]".to_string()
                                                            ))
                                                        ],
                                                        ConditionalMode::And
                                                    )),
                                                    Box::new(SeqRoutine::new(vec![
                                                        Box::new(CreateDeckRoutine::new(&"Meld [#]".to_string(), DeckVisibilityEvaluatable::new(false, true, Vec::new(), Vec::new()))),
                                                        Box::new(DealSpecificRoutine::new(&"[THISPLAYER]'s hand".to_string(), &"Meld [#]".to_string(), ChoiceLimit::Limited(3)))
                                                    ])),
                                                    CondRoutineMode::PostCond
                                                )
                                            )
                                        ],
                                        ChoiceLimit::Unlimited
                                    ))
                                ]))
                            ))
                        ))
                    ]))
                ))
            ]
        )
    );

    game
}