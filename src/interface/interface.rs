use std::io::{stdin, stdout, Write};

use crate::rules::routine::choice_routine::Choice;

fn take_input_line() -> String {
    let mut s = String::new();

    //Print prompt
    print!(">");
    stdout().flush();

    while let Err(e) = stdin().read_line(&mut s) {
        //Print error message and next prompt
        print!("Invalid input, please try again\n>");
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
        print!("({}) {}", i, choices[i].name());
        if i < l-1 {
            print!(" ");
        }
    }
    print!("\n>"); //Prompt
    stdout().flush();

    //Take user input
    let mut success = false;
    let mut ret = 0;
    while !success {
        if let Ok(idx) = take_input_line().parse::<usize>() {
            ret = idx;
            break;
        }
        else {
            println!("Invalid input format. Please input the integer corresponding with your choice.");
        }
    }

    ret
}