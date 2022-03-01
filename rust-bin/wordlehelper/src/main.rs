/*
* rust implementation of wordle helper.
* works by taking the frequency distribution of characters and their positions in a 5 letter word
* then trying to pick words where each character is the best possible character for that location
* if a picked word doesn't exist, it then changes a single letter and tries again.
* the letter that it changed is the letter that has another letter closest to the optimal letter
*
* ex. the best word is 'sares'. this is not a word.
*     the distance to the next letter for all of those letters are {s:500, a:300, r:50, e:600, s:900}.
*     the 'r' is the closest to the next letter. lets say this next letter is 't'. so that letter is changed.
*
*     the next word is 'sates'. this is not a word.
*     the distance to the next letter for all of those letters are {s:500, a:300, t:150, e:600, s:900}.
*     the catch is that the distance for 't' is not the distance between 't' and the next letter(lets say its 'i').
*     the distance for 't' is the distance between 'i' and 'r'. lets say 't' and 'r' is 50, and 'i' and 't' is 100.
*     the distances of 50 and 100 would be added for the new distance.
*
*     the next word is 'saies'. this is not a word.
*     the distance to the next letter for all of those letters are {s:500, a:300, i:350, e:600, s:900}.
*     'a' now has the smallest distance. 'a' is changed to the next letter and 'i' is changed back to the best option 'r'.
*
*     there is an infinite loop in this logic technically.
*     right after switching 'a', the 3rd position 'r' will have the lowest distance and will be switched in the next loop.
*     pushing the new 2nd letter back to the optimal 'a' and thus starting this example over.
*     the solution is 
*
*     this happens until a word is found. that word becomes the suggested guess.
*     once the board state is updated with the results of that guess, we can start the algorithm over with a set of known correct locations, known incorrect letters, and known letters with incorrect locations.
*/

use std::env;
use std::fs;
use std::collections::HashMap;



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
