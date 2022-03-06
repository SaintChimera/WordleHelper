/* 
* validates and answer and returns a vector with the results
*/

// return the results of the guess
pub fn determine_board_results(answer: &String, guess_word: &String) -> Vec<u8> {
    let answer_chars: Vec<char> = answer.chars().collect();
    let guess_word_chars: Vec<char> = guess_word.chars().collect();
    let mut state_vec = vec![0,0,0,0,0];

    // for every entry in the guess word:
    //   check if its in the right position, set the vector position to 2 if it is
    //   else check if its in the word at all, set the vecotr position to 1 if it is
    //   else set the position to 0, because the letter is not in the word
    for i in 0..guess_word_chars.len(){
        if guess_word_chars[i] == answer_chars[i]{
            state_vec[i] = 2;
        }
        else if answer_chars.contains(&guess_word_chars[i]){
            state_vec[i] = 1;
        }
        else {
            state_vec[i] = 0;
        }
    }

//    println!("{:?} : {:?}", guess_word_chars, state_vec);
    return state_vec;
}

