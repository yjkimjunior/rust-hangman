// note on notation
// :: accesses items of a module
// . accesses fields and methods of a struct

/*
    Basic Hangman Command Line Game - Made With Rust

    Game:
        - Start With Prompting User: Heads or Tails?
            - If user gets it right, they get to guess
            - else they let the computer guess
        - Either Way:
            - A word is chosen (for a computer from a preselected list, user can put whatever)
            - Reveal the word as _ _ _ _ _ _
            - Reveal the Hanging stand
            =========
            |       |
            |
            |
            |
            |
            - Each Turn:
                - Guesser guesses a letter
                - If letter in word:
                    - reveal letter in position _ a _ _ _ _
                - else:
                    - add a body part to the hangstands
                    - Order: Head, Neck, Body, Arms, Left Leg, Right Leg
                - If word complete: Guesser wins! Add point
                - If hangman complete: Chooser wins! Add point
            - End Game:
                - Show the score 1 - 0
                - Prompt user: Play Again?
*/

extern crate clap;
extern crate ctrlc;
use clap::{App};
use dialoguer::Input;
use rand::prelude::*;
// use std::env:: { current_dir, join_paths };
use std::fs::File;
use std::io:: { BufReader, BufRead };
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Player {
    Human,
    Computer
}

// struct Round {
//     score: uint64;
// }

// impl Round = {

// }

static ALPHABET_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];

fn main() {
    App::new("Hangman")
        .version("0.1.0")
        .author("YJ Kim <yjkimjunior@gmail.com>")
        .about("hangman cli game written in Rust")
        .get_matches();

    let mut words: Vec<String> = Vec::new();

    // FIXME: dont hard code the path
    let words_list_path = Path::new("/Users/yj/Developer/hangman/resources/words.txt");

    // FIXME: dont panic
    let reader = BufReader::new(File::open(words_list_path).unwrap());

    // Get all the valid words
    for line in reader.lines() {
        // FIXME: dont panic
        words.push(line.unwrap_or("something went wrong getting this line".to_string()));
    }

    // Use Random Number Gen to pick heads or tails (0 for heads, 1 for tails)
    let coin_flip_result = match rand::thread_rng().gen_range(0, 2) {
        0 => "H".to_string(),
        1 => "T".to_string(),
        _ => panic!("Coin flip fucked up")
    };

    // Prompt user for their choice H or T
    let coin_flip_guess: String = Input::new().with_prompt("Heads (H) or Tails (T)?").interact().unwrap();

    let game_word: String;
    let mut guesser: Player;

    if coin_flip_guess == coin_flip_result {
        // If Human choosing word - prompt and make sure it's in the word list
        guesser = Player::Computer;
        game_word = Input::new().with_prompt("Congrats you get to start! Pick a Word For The Game").interact().unwrap();
    } else {
        guesser = Player::Human;
        // If Computer choosing word - select a word for the game using random number
        let random_num: usize = rand::thread_rng().gen_range(0, words.len());
        game_word = words[random_num].clone();
    }

    let answer: Vec<char> = game_word.chars().collect();
    let mut correctGuesses: Vec<char> = Vec::new();
    let mut wrongGuesses: Vec<char> = Vec::new();

    // Loop time: Hangman State -> Guess -> Updated Hangman State -> Repeat
    // 6 wrong guesses permitted: head, body, 2 arms, 2 legs
    while correctGuesses != answer && wrongGuesses.len() < 6 {
        let guess: char;
        if guesser == Player::Human {
            // prompt human to guess a character
            guess = Input::new().with_prompt("What's your guess?").interact().unwrap();
        } else {
            // choose a random letter from alphabet with 1/26 probability
            guess = ALPHABET_LOWER[rand::thread_rng().gen_range(0, ALPHABET_LOWER.len())];
            // TODO: impl some basic strategies, e.g. guessing vowels first
        }
        // assigns index of the guessed char to Some(number) or None
        // FIXME: handle multiple of the same letter in correct answer
        let index = answer.iter().position(|&letter| letter == guess);

        match index {
            Some(i) => {
                &correctGuesses.insert(i, guess);
                println!("Correct Guesses: {:?}", correctGuesses);
            }
            _ => {
                &wrongGuesses.push(guess);
                println!("Wrong Guesses: {:?}", wrongGuesses);
            }
        }
    }

    // At this point the round is over. 
    // Either wrongGuesses.len() > 6 or correctGuess has been completed.
    if correctGuesses == answer {
        println!("Congratulations to the winner: {:?}! The word was: {:?}", guesser, answer);
    }

    if wrongGuesses.len() >= 6 {
        println!("You Suck! The correct answer was: {:?} and you guessed: {:?} but at least you got these correct: {:?}", answer, wrongGuesses, correctGuesses);
    }

    // switch player roles, display current score, start new round
    match guesser {
        Player::Human => {
            // switch player roles
            guesser = Player::Computer;
        },
        Player::Computer => {
            // switch player roles
            guesser = Player::Human;
        }
    }
}
