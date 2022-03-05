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
*     this happens until a word is found. that word becomes the suggested guess.
*     once the board state is updated with the results of that guess, we can start the algorithm over with a set of known correct locations, known incorrect letters, and known letters with incorrect locations.
*
*     there is an infinite loop in this logic technically.
*     right after switching 'a', the 3rd position 'r' will have the lowest distance and will be switched in the next loop.
*     pushing the new 2nd letter back to the optimal 'a' and thus starting this example over.
*     the solution is to maintain a vec denoting how many times a position has been changed. this is the sync table.
*     then using that to decide whether a position is changed back
*     a position is only changed back if its sync table value is larger than that of the value being changed.
*
{s:800, a:300, r:50, e:900, s:1000}   [0,0,0,0,0]
{s:800, a:300, t:250, e:900, s:1000}  [0,0,1,0,0]
{s:800, a:300, l:350, e:900, s:1000}  [0,0,2,0,0]
{s:800, b:400, r:50, e:900, s:1000}   [0,1,2,0,0]
{s:800, b:400, t:250, e:900, s:1000}  [0,1,3,0,0]
{s:800, b:400, l:350, e:900, s:1000}  [0,1,4,0,0]
{s:800, b:400, g:450, e:900, s:1000}  [0,1,5,0,0]
{s:800, c:500, r:50, e:900, s:1000}   [0,2,5,0,0]
{s:800, c:500, t:250, e:900, s:1000}   [0,2,6,0,0]
{s:800, c:500, l:350, e:900, s:1000}   [0,2,7,0,0]
{s:800, c:500, g:450, e:900, s:1000}   [0,2,8,0,0]
{s:800, c:500, p:550, e:900, s:1000}   [0,2,9,0,0]
{s:800, d:600, r:50, e:900, s:1000}   [0,3,9,0,0]
{s:800, d:600, t:250, e:900, s:1000}   [0,3,10,0,0]
{s:800, d:600, l:350, e:900, s:1000}   [0,3,11,0,0]
{s:800, d:600, g:450, e:900, s:1000}   [0,3,12,0,0]
{s:800, d:600, p:550, e:900, s:1000}   [0,3,13,0,0]
{s:800, d:600, u:1200, e:900, s:1000}   [0,3,14,0,0]
{s:800, e:650, r:50, e:900, s:1000}   [0,4,14,0,0]
{s:800, e:650, t:250, e:900, s:1000}   [0,4,15,0,0]
{s:800, e:650, l:350, e:900, s:1000}   [0,4,16,0,0]
{s:800, e:650, g:450, e:900, s:1000}   [0,4,17,0,0]
{s:800, e:650, p:550, e:900, s:1000}   [0,4,18,0,0]
{s:800, e:650, u:1200, e:900, s:1000}   [0,4,19,0,0]
{s:800, f:700, r:50, e:900, s:1000}   [0,5,19,0,0]
{s:800, f:700, t:250, e:900, s:1000}   [0,5,20,0,0]
{s:800, f:700, l:350, e:900, s:1000}   [0,5,21,0,0]
{s:800, f:700, g:450, e:900, s:1000}   [0,5,22,0,0]
{s:800, f:700, p:550, e:900, s:1000}   [0,5,23,0,0]
{s:800, f:700, u:1200, e:900, s:1000}   [0,5,24,0,0]
{s:800, g:750, r:50, e:900, s:1000}   [0,6,24,0,0]
*
*    an expansion on all of the above is to use previous answers as the input to the letter frequencies instead of the whole wordle list.
*    the whole wordle list has some crazy words in it. the answer list should only have reasonable words.
*    a filtered list of all wordle lists may be a good idea as well.
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
        let guess_word = player::get_word(&words,&distance_lists, &board_state, &answers);
        let _guess_word = player::suggest_word(&words,&distance_lists, &board_state, &answers);
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
            println!("guessed '{}' in {} guesses. {:?}",guess_word,loop_counter,guesses);
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
