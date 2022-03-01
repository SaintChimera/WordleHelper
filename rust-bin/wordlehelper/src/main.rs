// rust implementation of wordle helper. python implementation is in python-bin.

use std::env;
use std::fs;
use std::collections::HashMap;

// TODO: build omit vec

// TODO: build include hashmap

// TODO: suggest letters

// TODO: find words with letters

// TODO: get suggested word

// TODO: suggest word
fn suggest_word(words: &Vec<&str>, board_state: &HashMap<&str,Vec<u8>>) -> String {
    // build omit and include lists
    //let omit: Vec<&str> = build_omit_list(board_state)
    //let include: HashMap<char, Vec<i8>> = build_include_list(board_state)

    let word: String = words[0].to_string();
    return word;
}

// get the word list, suggest word to player, get board state update from player.
// handles errors
fn main() {
    // get words in file
    let args: Vec<String> = env::args().collect();
    let word_file = &args[1];
    let words_s = fs::read_to_string(word_file);
    let words_s = match words_s {
        Ok(words_s) => words_s,
        Err(error) => panic!("Could not open word file: {:?}", error)
    };
    // split to vec of str's
    let words: Vec<&str> = words_s.split("\n").collect();

    // board state tracks all guesses and the results of those guesses.
    // value is a hot encoding where 0 is a miss, 1 is an incorrect position, 2's are correct positions.
    let mut board_state: HashMap<&str,Vec<u8>> = HashMap::new();

    // get our first word
    let word = suggest_word(&words, &board_state);
    println!("{:?}",word);
}
