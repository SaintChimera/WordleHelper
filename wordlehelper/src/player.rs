/*
* guesses words for wordle. depends on main. methods are explained in blog post
*/

use std::io;
use std::collections::HashMap;
use std::collections::HashSet;

// get a collection of letters that the guess word should use.
pub fn suggest_letters(words: &HashSet<&str>, loop_counter: &usize) -> Vec<char>{
    let mut letters_freq: HashMap<char,usize> = HashMap::new();
    
    // get the letter frequencies
    for word in words.into_iter(){
        let letters: Vec<char> = word.chars().collect();
        for i in 0..letters.len(){
            let letter = letters[i];
            let letter_freq_entry = letters_freq.entry(letter).or_insert(0);
            *letter_freq_entry += 1;
        }
    }

    // sort that row by smallest to largest
    let mut sorted_row: Vec<(char,usize)> = letters_freq.into_iter().collect();
    sorted_row.sort_by_key(|a| a.1);
    sorted_row.reverse();

    // take 5 letters according to the loop count.
    let suggest_letters = vec![sorted_row[0+(loop_counter*5)].0,
                           sorted_row[1+(loop_counter*5)].0,
                           sorted_row[2+(loop_counter*5)].0,
                           sorted_row[3+(loop_counter*5)].0,
                           sorted_row[4+(loop_counter*5)].0];

    return suggest_letters;
}

// get the frequencies of each letter in their positions
// use the omit list and include list to force letters in or out of their positions
pub fn get_letter_frequencies(words: &HashSet<&str>, board_state: &HashMap<String,Vec<u8>>) -> HashMap<char,Vec<usize>>{
    // omit list, used to indicate letters that are definitely not in the set and in the wrong position
    let omit_list = build_omit_list(board_state);
    // include list, used to indicate letters that are definitely in the right position.
    let include_list = build_include_list(board_state);

    let mut letter_dist: HashMap<char,Vec<usize>> = HashMap::new();
    // for each word in our word list
    for word in words.into_iter(){ 
        let letters: Vec<char> = word.chars().collect();
        // i is the position we're analyzing in a word
        for i in 0..letters.len(){
            let letter = letters[i];
            // omit first so that we dont mess up our include list, which is the more accurate guess.
            // if a letter is in the omit list
            if omit_list.contains_key(&letter) {
                let omit_list_locs = match omit_list.get(&letter){
                    Some(a) => a,
                    None => panic!("Value is not in key like we just validated. Data corruption has occured.")
                };
                // if the position we're analyzing is in the omit vec for this letter
                if omit_list_locs.contains(&i) || omit_list_locs.contains(&6) {
                    let letter_l = letter_dist.entry(letter).or_insert(vec![0,0,0,0,0]);
                    letter_l[i] = 0; // hard set the location to no occurences
                    continue
                }
            }
            // if a letter is in the include list
            if include_list.contains_key(&letter) {
                let include_list_locs = match include_list.get(&letter){
                    Some(a) => a,
                    None => panic!("Value is not in key like we just validated. Data corruption has occured.")
                };
                // if the include position for that letter is the position we're analyzing
                if include_list_locs.contains(&i){
                    let letter_l = letter_dist.entry(letter).or_insert(vec![0,0,0,0,0]);
                    letter_l[i] = 200000; // hard set the location super high so that the letter doesn't get rotated
                    continue
                }
            }
            // if we haven't quit by this point then our letter and position isn't in the omit and include lists
            // so just add the letter
            let letter_l = letter_dist.entry(letter).or_insert(vec![0,0,0,0,0]);
            letter_l[i] = letter_l[i] + 1 // increment the position in the existing vec for that letter
        }
    }

    return letter_dist;
}

// omit list. a letter and position tuple, where the position is where to omit the letter from.
// a position >5 indicates an omit from every position
fn build_omit_list(board_state: &HashMap<String,Vec<u8>>) -> HashMap<char,Vec<usize>> {
    let mut omit_list: HashMap<char,Vec<usize>> = HashMap::new();
    // for each play on the game board
    for (guess,result) in board_state.iter(){
        let guess_split: Vec<char> = guess.chars().collect();
        for i in 0..guess_split.len(){ // guess_split and result should be the same length
            // 0 indicates a guess letter is not in the string at all.
            if result[i] == 0 {
                omit_list.insert(guess_split[i],vec![6]);
            } 
            // 1 indicates a guess letter is in the string, but not in the right position
            // so omit it from the specific position
            else if result[i] == 1 {
                let omit_list_l = omit_list.entry(guess_split[i]).or_insert(vec![]);
                omit_list_l.push(i)
            }
        }
    }

    return omit_list;
}

// include list. a letter and a position tuple, where the position is where to put the letter.
// conceptually an inverse omit list, where all other letters are removed, and the freq is set really high.
fn build_include_list(board_state: &HashMap<String,Vec<u8>>) -> HashMap<char,Vec<usize>> {
    let mut include_list: HashMap<char,Vec<usize>> = HashMap::new();
    // for each play on the game board
    for (guess,result) in board_state.iter(){
        let guess_split: Vec<char> = guess.chars().collect();
        for i in 0..guess_split.len(){ // guess_split and result should be the same length
            // 2 indicates a guess letter is in the guess location
            if result[i] == 2 {
                let include_list_l = include_list.entry(guess_split[i]).or_insert(vec![]);
                include_list_l.push(i)
            } 
        }
    }
    
    return include_list;
}

// a list of letters which are in the file but not in the correct position.
// the omit list already takes care of making sure these letters are not in the wrong position
// the include list takes care of letters in the correct position.
// so this just needs to be a list of letters that were in the string but in the wrong spot.
// position will work itself out from the omit list and include list
fn build_required_list(board_state: &HashMap<String,Vec<u8>>) -> Vec<char> {
    let mut required_letters: Vec<char> = Vec::new();
    // for each play on the game board
    for (guess,result) in board_state.iter(){
        let guess_split: Vec<char> = guess.chars().collect();
        for i in 0..guess_split.len(){ // guess_split and result should be the same length
            // 0 indicates a guess letter is not in the string at all.
            if result[i] == 1 {
                required_letters.push(guess_split[i]); // include the letter
            } 
        }
    }
    
    return required_letters
}

// returns a vector where each position is a distance list for that position in the string
pub fn get_distance_list(letter_dist: &HashMap<char,Vec<usize>>) -> Vec<Vec<(char,usize)>>{
    // iterate over hashmap pulling out the vec's values into separate hashmaps. push those to a vec to be our distance lists.
    let mut distance_lists: Vec<Vec<(char,usize)>> = Vec::new();
    for i in 0..5{
        // unpack a hashmap of just distributions for 1 specific position
        let mut ret_row: HashMap<char,usize> = HashMap::new();
        for (letter,list) in letter_dist.iter(){
            ret_row.insert(*letter,list[i]);
        }


        // sort that row by smallest to largest
        let mut sorted_row: Vec<(char,usize)> = ret_row.into_iter().collect();
        sorted_row.sort_by_key(|a| a.1);
        sorted_row.reverse();
        // artificial entry of an ending letter to not rotate past
        sorted_row.push(('.',0));

        // iterate over frequency row and build a distance list
        let mut distance_list: Vec<(char,usize)> = Vec::new();
        // all distance are with reference to the optimal
        if sorted_row.len() <= 0{
            panic!("sorted_row size is 0, which is not possible.")
        }
        let (_optimal_letter,optimal_freq) = sorted_row[0];
        for i in 0..sorted_row.len() {
            let distance: usize;
            let (letter,freq) = sorted_row[i];
            // if this is the last letter or if the frequency is 0 indicating it should be skipped
            if i == (sorted_row.len()-1) || freq == 0 {
                distance = 1000000; // something really high that wont be rotated.
            } else { 
                let (_next_letter,_next_freq) = sorted_row[i+1]; // grab the next letters frequence, store it under this letter.
                distance = optimal_freq - freq;
            }
            distance_list.push((letter,distance));
        }

        distance_lists.push(distance_list);
    }


    // return looks like distance_lists<distance_list<letter,distance>>
    return distance_lists;
}

// take each word in words, assign a distance score to it according to the distance lists, check if its in answers, return the lowest distance score word. this is the best guess
pub fn suggest_word(words: &HashSet<&str>, distance_lists: &Vec<Vec<(char,usize)>>, board_state:&HashMap<String,Vec<u8>>, answers: &Vec<&str>, letters: Vec<char>) -> String{

    // hashmap to store each word and its distance value
    let mut word_distances: HashMap<&str,usize> = HashMap::new();

    // for every word, split it into its characters, add the distance of each letter to the accumulator, store the word and its score.
    for word in words.iter(){
        let word_letters: Vec<char> = word.chars().collect();
        
        // accumulator will be the total distance for a word.
        let mut accumulator = 0;
        for i in 0..word_letters.len(){
            let letter = word_letters[i];
            // unpack the positions distance list;
            let distance_list = &distance_lists[i];
            // get the distance for the current letter
            let mut distance = 0;
            for distance_pair in distance_list.iter(){
                if distance_pair.0 == letter{
                    distance = distance_pair.1;
                }
            }
            // add the distance to the accumulator
            accumulator += distance;
        }

        // assign the word and distances to the hashmap
        word_distances.insert(word,accumulator);
    }
    word_distances.remove("");

    // if we are guessing based on simple letter frequencies, there will be a letters vec.
    // if we are guessing based on letter frequencies and positions, then we need to build a list of letters to include
    let mut required_letters: Vec<char> = Vec::new();
    if letters.len() > 0{
        required_letters = letters;
//        required_letters = build_required_list(board_state);
    }
    else {
        required_letters = build_required_list(board_state);
    }
    loop {
        let guess = match word_distances.iter().min_by_key(|entry| entry.1){
            Some(a) => a,
            None => panic!("No guess was found in get_word")
        };

        let guess_word = guess.0.to_string();
        let guess_word_vec: Vec<char> = guess_word.chars().collect();

        let mut valid_guess: bool = true;

        // make sure our word has all the required letters
        for required_letter in &required_letters{
            if !guess_word_vec.contains(required_letter){
                valid_guess = false;
            }
        }

        // make sure our first word does not have duplicate letters        
        if board_state.len() <= 0 {
            for letter in guess_word_vec.iter(){
                let mut guess_word_vec_clone = guess_word_vec.clone();
                let index = guess_word_vec_clone.iter().position(|x| x == letter).unwrap();
                guess_word_vec_clone.remove(index);
                if guess_word_vec_clone.contains(&letter){
                    valid_guess = false;
                }
            }
        }
        
        if valid_guess {
            return guess_word;
        }
        else { 
            word_distances.remove(guess_word.as_str());
        }
    }
}

// get board results from user
pub fn get_board_results() -> Vec<u8> {
    // get input
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // trim the newline
    if input.ends_with('\n') {
        input.pop();
    }

    // complain if the string isn't 5 characters long
    if input.len() != 5{
        panic!("Input is either too long or too short")
    }

    // parse the input into an array of integers
    let split_input_raw: Vec<&str> = input.split("").collect();
    let mut state_vec: Vec<u8> = Vec::new();
    for entry in split_input_raw.iter(){
        if entry.len() > 0{
            // parse string like "0", "1", or "2" and handle errors
            let state_entry = match entry.parse::<u8>() {
                Ok(a) => a,
                Err(_) => panic!("Could not parse input number. make sure you're doing it like '00120'")
            };
            state_vec.push(state_entry);
        }
    }
    return state_vec
}

