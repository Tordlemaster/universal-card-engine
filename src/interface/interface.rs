use std::{io::{stdin, stdout, Write}, num::ParseIntError};

use crate::{interface::deck_printing::card_to_str, rules::{deck::{CardSetData, Deck}, routine::choice_routine::{Choice, ChoiceLimit}}};

pub fn take_input_line() -> String {
    let mut s = String::new();

    //Print prompt
    print!(">");
    stdout().flush();

    while let Err(e) = stdin().read_line(&mut s) {
        //Print error message and next prompt
        print!("Invalid input, please try again.\n>");
        stdout().flush();
    }

    //Remove newline characters
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }

    //Return
    s
}

pub fn choice_interface(choices: &Vec<Choice>) -> usize {
    //Print each option with an ordinal
    let l = choices.len();
    for i in 0..l {
        print!("({}) {}", i+1, choices[i].name());
        if i < l-1 {
            print!(" ");
        }
    }
    println!();
    //print!("\n>"); //Prompt
    //stdout().flush();

    //Take user input
    let mut success = false;
    let mut ret = 0;
    while !success {
        if let Ok(idx) = take_input_line().parse::<usize>() {
            ret = idx-1;
            break;
        }
        else {
            println!("Invalid input format. Please input the integer corresponding with your choice.");
        }
    }

    ret
}

pub fn card_subset_interface(deck: &Deck, deck_name: &String, n: ChoiceLimit, card_set_data: &CardSetData) -> Vec<usize> {
    match n {
        ChoiceLimit::Limited(n) => {
            println!("Select {} cards from {} by typing their indices separated by spaces:", n, deck_name);
            for i in 0..deck.cards().len() {
                print!("({}) {}  ", i, card_to_str(&deck.cards()[i], card_set_data));
            }
            println!();
            
            //Take input, handle invalid input
            let mut valid = false;
            let mut ret: Vec<usize> = Vec::new();
            while !valid {
                let input = take_input_line();
                if let Ok(r) = _csi(&input) {
                    if r.len() == n {
                        if *r.iter().max().unwrap() < deck.len() {
                            valid = true;
                            ret = r;
                        }
                        else {
                            println!("Card indices out of range, please try again.")
                        }
                    }
                    else {
                        println!("Wrong number of options, please try again.");
                    }
                }
                else {
                    println!("Invalid input, please try again.");
                }
            }
            ret
        }
        ChoiceLimit::Unlimited => {
            println!("Select a subset of cards from {} by typing their indices separated by spaces:", deck_name);
            for i in 0..deck.cards().len() {
                print!("({}) {}  ", i, card_to_str(&deck.cards()[i], card_set_data));
            }
            println!();

            let mut valid = false;
            let mut ret: Vec<usize> = Vec::new();
            while !valid {
                let input = take_input_line();
                if let Ok(r) = _csi(&input) {
                    if r.len() <= deck.len() && r.len() > 0 {
                        if *r.iter().max().unwrap() < deck.len() {
                            valid = true;
                            ret = r;
                        }
                        else {
                            println!("Card indices out of range, please try again.")
                        }
                    }
                    else {
                        println!("Wrong number of options, please try again.");
                    }
                }
                else {
                    println!("Invalid input, please try again.");
                }
            }
            ret
        }
    }
}

fn _csi(input: &String) -> Result<Vec<usize>, ParseIntError> {
    input.split_whitespace().map(|s| s.parse::<usize>()).collect()
}