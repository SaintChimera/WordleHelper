/*
* rust implementation of wordle helper.
* works by taking the frequency distribution of characters and their positions in a 5 letter word
* then trying to pick words where each character is the best possible character for that location
*
*/

use std::env;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

mod player;
mod game;

// receive words, answers, and day. pick answer word. begin loop of calling player, validating guess, and returning guess results
fn automated(words: &HashSet<&str>, answers: &Vec<&str>, day: &usize) {
    // grab a word to be the answer
    let answer = answers[day.clone()];

    // trim the answers to only have old answers. not current or future ones
    let mut answers = answers.to_vec();
    answers.resize(day.clone(),&"");

    // board state tracks all guesses and the results of those guesses.
    // value is a hot encoding where 0 is a miss, 1 is an incorrect position, 2's are correct positions.
    let mut board_state: HashMap<String,Vec<u8>> = HashMap::new();

    // loop counter keeps track of how many guesses it took
    let mut loop_counter = 0;
    // guesses keeps track of all guesses, to be printed to the user
    let mut guesses: Vec<String> = Vec::new();
    // loop and check answer
    loop {
        // update loop counter to match guess count
        loop_counter += 1;

        // get letter frequencies
        let letter_dist = player::get_letter_frequencies(&words, &board_state);

        // get distance lists for each row
        let distance_lists = player::get_distance_list(&letter_dist);

        // get a word
        let guess_word = player::suggest_word(&words,&distance_lists, &board_state, &answers);
        guesses.push(guess_word.clone());
        if guess_word == "".to_string(){
            println!("failed to guess word {:?}",guesses);
            break
        }

        // get board results
        let state_vec = game::determine_board_results(&answer.to_string(), &guess_word);

        // quit if we're successful
        if state_vec[0] == 2 &&
           state_vec[1] == 2 &&
           state_vec[2] == 2 &&
           state_vec[3] == 2 &&
           state_vec[4] == 2 {
//            println!("day {} : guessed '{}' in {} guesses. {:?}",day,guess_word,loop_counter,guesses);
            println!("{},{}",day,loop_counter);
            break
        }

        // update the board state
        board_state.insert(guess_word.clone(),state_vec);
    }
}

fn interactive(words: &HashSet<&str>, answers: &Vec<&str>, day: &usize) {
    // trim the answers to only have old answers. not current or future ones
    let mut answers = answers.to_vec();
    answers.resize(day.clone(),&"");

    // board state tracks all guesses and the results of those guesses.
    // value is a hot encoding where 0 is a miss, 1 is an incorrect position, 2's are correct positions.
    let mut board_state: HashMap<String,Vec<u8>> = HashMap::new();

    // loop with user input
    loop {
        // get letter frequencies
        let letter_dist = player::get_letter_frequencies(&words, &board_state);

        // get distance lists for each row
        let distance_lists = player::get_distance_list(&letter_dist);

        // suggest a word
        let guess_word = player::suggest_word(&words,&distance_lists, &board_state, &answers);
        if guess_word == "".to_string(){
            println!("No more words left to guess. The answer word is not in the list.");
            break
        }
        println!("guess '{}'", guess_word);

        // get board results
        let state_vec = player::get_board_results();

        // quit if we're successful
        if state_vec[0] == 2 &&
           state_vec[1] == 2 &&
           state_vec[2] == 2 &&
           state_vec[3] == 2 &&
           state_vec[4] == 2 {
            println!("Congratulations.");
            break()
        }

        // update the board state
        board_state.insert(guess_word.clone(),state_vec);


    }
}

// get the word list, suggest word to player, get board state update from player.
fn main() {
    let args: Vec<String> = env::args().collect();
    // sanity check that we have enough args
    if args.len() < 4{
        println!("Not enough args, run like $ ./wordlehelper ../../words/wordle_words.txt ../../words/ny_times_answers.txt 257 i");
        return;
    }
    
    // get words in file
    let word_file = &args[1];
    let words_s = fs::read_to_string(word_file);
    let words_s = match words_s {
        Ok(words_s) => words_s,
        Err(error) => panic!("Could not open word file: {:?}", error)
    };
    // split to hashset of str's
    let words: HashSet<&str> = words_s.split("\n").collect();

    // get answer list so that we can exclude previous answers from our guesses.
    let answer_file = &args[2];
    let answer_s = fs::read_to_string(answer_file);
    let answer_s = match answer_s {
        Ok(a) => a,
        Err(error) => panic!("Could not open answer file: {:?}", error)
    };

    // get the day that we're playing to trim down the answer list.
    let day_a = &args[3];
    let day = match day_a.parse::<usize>() {
        Ok(a) => a,
        Err(error) => panic!("Could not parse day: {:?}", error)
    };
    // reduce to a vec of old answers according to the day
    let answers: Vec<&str> = answer_s.split("\n").collect();

    let mode = &args[4];

    if mode == &"a"{
        if day == 10000{
//            println!("day,guesses");
            for i in 0..answers.len(){
                automated(&words, &answers, &i);
            }
        }
        else {
            automated(&words, &answers, &day);
        }
    }
    else if mode == &"i" {
        println!("answer is '{}'",answers[day]);
        interactive(&words, &answers, &day);
    }
    else {
        println!("Invalid game mode. Please use 'a' or 'i'.")
    }
}
