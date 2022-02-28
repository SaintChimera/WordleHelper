// rust implementation of wordle helper. python implementation is in python-bin.

use std::env;
use std::fs;

// TODO: build omit vec

// TODO: build include hashmap

// TODO: suggest letters

// TODO: find words with letters

// TODO: get suggested word

// TODO: suggest word

// get the word list, suggest word to player, get board state update from player.
// handles errors
fn main() {
    // get word file
    let args: Vec<String> = env::args().collect();
    let word_file = &args[1];
    
    let words = fs::read_to_string(word_file);
    let words = match words {
        Ok(words) => words,
        Err(error) => panic!("Could not open word file: {:?}", error)
    };
    
}
