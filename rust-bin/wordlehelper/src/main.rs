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


fn get_letter_frequencies(words: &Vec<&str>) -> HashMap<char,Vec<usize>>{
    let mut letter_dist: HashMap<char,Vec<usize>> = HashMap::new();
    for word in words.into_iter(){
        let letters: Vec<char> = word.chars().collect();
        for i in 0..letters.len(){
            let letter = letters[i];
            let letter_l = letter_dist.entry(letter).or_insert(vec![0,0,0,0,0]);
            letter_l[i] = letter_l[i] + 1
        }
    }

    return letter_dist;
}

// TODO: build sorted frequency distance lists for each position in the word
// returns a vector where each position is a distance list for that position in the string
fn get_distance_list(letter_dist: &HashMap<char,Vec<usize>>) -> Vec<Vec<(char,usize)>>{
//fn get_distance_list(letter_dist: &HashMap<char,Vec<usize>>) {
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
        let (optimal_letter,optimal_freq) = sorted_row[0];
        for i in 0..sorted_row.len() {
            let mut distance = 0;
            let (letter,freq) = sorted_row[i];
            distance = optimal_freq - freq;
            if i == (sorted_row.len()-1){
                distance = 100000; // something really high that wont be rotated.
            }
            distance_list.push((letter,distance));
        }
        distance_lists.push(distance_list);
    }

    // return looks like distance_lists<distance_list<letter,distance>>
    return distance_lists;
}

// TODO: used distance lists to find a word.

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

    // get letter frequencies
    let letter_dist = get_letter_frequencies(&words);

    // get distance lists for each row
    let distance_list = get_distance_list(&letter_dist);

}
