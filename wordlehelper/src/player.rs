/*
* guesses words for wordle. depends on main. methods are explained in main
*/

use std::io;
use std::process;
use std::collections::HashMap;
use std::collections::HashSet;


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
                    letter_l[i] = 200000; // hard set the location super high
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
//fn build_include_list(board_state: &HashMap<String,Vec<u8>>) -> (Vec<char>,Vec<usize>) {
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
        let mut ret_row: HashMap<char,usize> = HashMap::new();
        for (letter,list) in letter_dist.iter(){
            ret_row.insert(*letter,list[i]);
        }

        let mut sorted_row: Vec<(char,usize)> = ret_row.into_iter().collect();
        sorted_row.sort_by_key(|a| a.1);
        sorted_row.reverse();

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
                distance = 100000; // something really high that wont be rotated.
            } else { 
                let (_next_letter,next_freq) = sorted_row[i+1]; // grab the next letters frequence, store it under this letter.
                distance = optimal_freq - next_freq;
            }
            distance_list.push((letter,distance));
        }

        distance_lists.push(distance_list);
    }

    // return looks like distance_lists<distance_list<letter,distance>>
    return distance_lists;
}

//  use distance lists to find a word.
pub fn suggest_word(words: &HashSet<&str>, distance_lists: &Vec<Vec<(char,usize)>>, board_state:&HashMap<String,Vec<u8>>, answers: &Vec<&str>) -> String{
    // letter_movement keeps track of which letter positions are moving the most and ensures they keep moving
    let mut letter_movement = [0,0,0,0,0];
    // letter_grab keeps track of which positions to look at and grab from in the distance lists.
    let mut letter_grab = [0,0,0,0,0];

    // unpack distance lists
    let first_distance_list = &distance_lists[0];
    let second_distance_list = &distance_lists[1];
    let third_distance_list = &distance_lists[2];
    let fourth_distance_list = &distance_lists[3];
    let fifth_distance_list = &distance_lists[4];

    // loop
    loop {
        // check if we overflow an entry in the distance list
        // this means we couldn't find a valid word and we need to quit.
        if letter_grab[0] >= first_distance_list.len() ||
           letter_grab[1] >= second_distance_list.len() || 
           letter_grab[2] >= third_distance_list.len() || 
           letter_grab[3] >= fourth_distance_list.len() || 
           letter_grab[4] >= fifth_distance_list.len(){
            return "".to_string();
        }

        // generate a word from the top of the distance list
        let guess_word_vec = vec![
                            first_distance_list[letter_grab[0]].0,
                            second_distance_list[letter_grab[1]].0,
                            third_distance_list[letter_grab[2]].0,
                            fourth_distance_list[letter_grab[3]].0,
                            fifth_distance_list[letter_grab[4]].0
                            ];

        // tracks whether the guess word is valid or not
        // default to valid, and attempt to prove its invalid with the required_letters loop
        let mut valid_guess: bool = true;

        // get a list of letters that were in the word but not in the correct position, "1's"
        // if the requred letter is not in the generated word, rotate a letter in the word at try again.
        let required_letters = build_required_list(board_state);
        for required_letter in required_letters{
            if !guess_word_vec.contains(&required_letter){
                valid_guess = false;
            }
        }
        
        // make guess string
        let guess_word: String = guess_word_vec.into_iter().collect();

        // check if the word has been an answer in the past. dont guess it if it has been an answer.
        if answers.contains(&guess_word.as_str()){
            valid_guess = false;
        }
    
        // check if that word is in words. return it if it is.
        if valid_guess && words.contains(&guess_word.as_str()){
            return guess_word.to_string();
        } else {
            // otherwise find the letter with the lowest distance and shift it. update letter_movement.
            // build a vector of the current distances
            let minvec = vec![
                            first_distance_list[letter_grab[0]].1,
                            second_distance_list[letter_grab[1]].1,
                            third_distance_list[letter_grab[2]].1,
                            fourth_distance_list[letter_grab[3]].1,
                            fifth_distance_list[letter_grab[4]].1
                            ];
            let minimum_distance = match minvec.iter().min() {
                Some(a) => a,
                None => panic!("No minimum distance found in suggest_word")
            };
            let movement_position = match minvec.iter().position(|&x| x == *minimum_distance){
                Some(a) => a,
                None => panic!("No movement position found in suggest_word")
            };

            // now "move" that letter
            letter_grab[movement_position] += 1;
            letter_movement[movement_position] += 1;

            // find letters that need to move back to the optimal, because its a higher movement letter
            for i in 0..5{
                if i != movement_position && letter_movement[i] >= letter_movement[movement_position]{
                    letter_grab[i] = 0;
                }
            }
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

