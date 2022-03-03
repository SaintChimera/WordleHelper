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
use std::io;
use std::collections::HashMap;
use std::collections::HashSet;


// get the frequencies of each letter in their positions
// use the omit list and include list to force letters in or out of their positions
fn get_letter_frequencies(words: &HashSet<&str>, board_state: &HashMap<String,Vec<u8>>) -> HashMap<char,Vec<usize>>{
    // omit list, used to indicate letters that are definitely not in the set and in the wrong position
    let (omit_letters,omit_positions) = build_omit_list(board_state);
    // include list, used to indicate letters that are definitely in the right position.
    // TODO: as of right now, we ignore when a character is in the string but not in the correct position.
    //       right now it just gets added to the guess position in the omit list, so that it doesn't end up in that position again.
    let (include_letters,include_positions) = build_include_list(board_state);

    let mut letter_dist: HashMap<char,Vec<usize>> = HashMap::new();
    for word in words.into_iter(){ 
        let letters: Vec<char> = word.chars().collect();
        for i in 0..letters.len(){
            let letter = letters[i];
            if include_letters.contains(&letter) {
                let include_letter_loc = match include_letters.iter().position(|&x| x == letter){
                    Some(a) => a,
                    None => panic!("No movement position found in suggest_word")
                };
                // if our correct positions letter is in the position that we're analyzing.
                // set that correct position letter as the only letter for that position, with a massive frequency.
                if include_positions[include_letter_loc] == i{ 
                    let letter_l = letter_dist.entry(letter).or_insert(vec![0,0,0,0,0]);
                    letter_l[i] = 200000 // hard set the location super high
                }
            }
            else if omit_letters.contains(&letter) {
                let omit_letter_loc = match omit_letters.iter().position(|&x| x == letter){
                    Some(a) => a,
                    None => panic!("No movement position found in suggest_word")
                };
                // skip insertion since this letter has been deemed not in the answer word
                if omit_positions[omit_letter_loc] > 5 {
                    continue;
                }
                // skip insertion since this letter is not in this position in the answer word
                else if omit_positions[omit_letter_loc] < 5 && omit_positions[omit_letter_loc] == i {
                    continue;
                }
                // add the letter since the omit list doesn't omit it from this position
                else {
                    let letter_l = letter_dist.entry(letter).or_insert(vec![0,0,0,0,0]);
                    letter_l[i] = letter_l[i] + 1 // increment the position in the existing vec for that letter
                }
            }
            // add the letter since its not in the omit list
            else {
                let letter_l = letter_dist.entry(letter).or_insert(vec![0,0,0,0,0]);
                letter_l[i] = letter_l[i] + 1 // increment the position in the existing vec for that letter
            }
        }
    }

    return letter_dist;
}

// omit list. a letter and position tuple, where the position is where to omit the letter from.
// a position >5 indicates an omit from every position
fn build_omit_list(board_state: &HashMap<String,Vec<u8>>) -> (Vec<char>,Vec<usize>) {
    let mut omit_letters: Vec<char> = Vec::new();
    let mut omit_positions: Vec<usize> = Vec::new();
    // for each play on the game board
    for (guess,result) in board_state.iter(){
        let guess_split: Vec<char> = guess.chars().collect();
        for i in 0..guess_split.len(){ // guess_split and result should be the same length
            // 0 indicates a guess letter is not in the string at all.
            if result[i] == 0 {
                omit_letters.push(guess_split[i]); // include the letter
                omit_positions.push(6); // a value > 5 means omit from all positions
            } 
            // 1 indicates a guess letter is in the string, but not in the right position
            else if result[i] == 1 {
                omit_letters.push(guess_split[i]); // a value > 5 means omit from all positions
                omit_positions.push(i); // a value > 5 means omit from all positions
            }
        }
    }

    return (omit_letters,omit_positions);
}

// include list. a letter and a position tuple, where the position is where to put the letter.
// conceptually an inverse omit list, where all other letters are removed, and the freq is set really high.
fn build_include_list(board_state: &HashMap<String,Vec<u8>>) -> (Vec<char>,Vec<usize>) {
    let mut include_letters: Vec<char> = Vec::new();
    let mut include_positions: Vec<usize> = Vec::new();
    // for each play on the game board
    for (guess,result) in board_state.iter(){
        let guess_split: Vec<char> = guess.chars().collect();
        for i in 0..guess_split.len(){ // guess_split and result should be the same length
            // 0 indicates a guess letter is not in the string at all.
            if result[i] == 2 {
                include_letters.push(guess_split[i]); // include the letter
                include_positions.push(i); // a value > 5 means omit from all positions
            } 
        }
    }
    
    return (include_letters,include_positions)
}

// a list of letters which are in the file but not in the correct position.
// the omit list already takes care of making sure these letters are not in the wrong position
// so this just needs to be a list of letters that need to be in the string, position will work itself out.
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
fn get_distance_list(letter_dist: &HashMap<char,Vec<usize>>) -> Vec<Vec<(char,usize)>>{
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
            let (letter,_freq) = sorted_row[i];
            if i == (sorted_row.len()-1){ // if this is the last
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
fn suggest_word(words: &HashSet<&str>, distance_lists: &Vec<Vec<(char,usize)>>, board_state:&HashMap<String,Vec<u8>>) -> String{
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

        let guess_word: String = guess_word_vec.into_iter().collect();
    
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
fn get_board_results() -> Vec<u8> {
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

// get the word list, suggest word to player, get board state update from player.
fn main() {
    // get words in file
    let args: Vec<String> = env::args().collect();
    let word_file = &args[1];
    let words_s = fs::read_to_string(word_file);
    let words_s = match words_s {
        Ok(words_s) => words_s,
        Err(error) => panic!("Could not open word file: {:?}", error)
    };
    // split to hashset of str's
    let words: HashSet<&str> = words_s.split("\n").collect();

    // board state tracks all guesses and the results of those guesses.
    // value is a hot encoding where 0 is a miss, 1 is an incorrect position, 2's are correct positions.
    let mut board_state: HashMap<String,Vec<u8>> = HashMap::new();

    // loop with user input
    loop {
        // get letter frequencies
        let letter_dist = get_letter_frequencies(&words, &board_state);

        // get distance lists for each row
        let distance_lists = get_distance_list(&letter_dist);

        // get a word
        let guess_word = suggest_word(&words,&distance_lists, &board_state);
        println!("guess '{}'", guess_word);

        // get board results
        let state_vec = get_board_results();
        
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
