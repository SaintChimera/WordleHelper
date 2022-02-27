import os, sys

# function to find the most common 5 letters in the word list.
def suggest_letters(words, omit=None, include={}):
    # make a dict with the number of occurences for ever letter.
    letter_dist = {}
    for word in words:
        for letter in word:
            if letter in letter_dist:
                letter_dist[letter] += 1
            else:
                letter_dist[letter] = 1
    
    # return the top 5
    letters = list(include.keys())
    for i in range(len(letters),5):
        largest_key = ''
        largest_value = 0
        for key,value in letter_dist.items():
            if key in omit:
                continue
            if value > largest_value:
                largest_key = key
                largest_value = value
        letters.append(largest_key)
        letter_dist.pop(largest_key)
    letters = ''.join(a for a in letters)
    return letters

# finds all possible words according to the letters list, not considering the include list
# TODO: if suggestable_words is empty every, we may need to change the implementation to allow only 4 letters to match.
def _find_words(words, letters):
    suggestable_words = []
    for word in words:
        if all(letter in word for letter in letters):
            suggestable_words.append(word)
    return suggestable_words

# function to suggest a word according to the board state and the 5 most common letters in the word list.
# TODO: implement include list for new positions.
def suggest_word(words, letters, include={}):
    suggestable_words = _find_words(words, letters)
    # just return the first if we have no include dict
    candidate_word = ''
    if len(include) == 0:
        candidate_word = suggestable_words[0]
        return candidate_word
    # find word with proper include positions and improper include positions
    for suggestable_word in suggestable_words:
        for letter,position in include.items():
            # first check if an include letter is in our suggestable_word. because it has to be
            suggestable_word_letter_loc = suggestable_word.find(letter)
            if suggestable_word_letter_loc < 0:
                break
            # if the letter is a proper location letter and the letter was not found in that proper location.
            if position > 0:
                if position != suggestable_word_letter_loc:
                    break
            # if the letter is an improper location letter and the letter was found in the same location.
            if position < 0:
                if (position*-1) == suggestable_word_letter_loc:
                    break
            
    

# function to build an omit list with the board state.
def build_omit_list(board_state):
    if len(board_state) == 0:
        return None
    omit = []
    for word,hot_l in board_state.items():
        for i in range(len(hot_l)):
            hot = hot_l[i]
            if hot == 0:
                omit.append(word[i])
            elif hot == 1 or hot == 2:
                continue
            else:
                print(f"invalid hot encoding {hot} for {word}")
                exit()
    return omit

# function to build an include list with the board list
# include dict has a key of the letter, value of the desired position. a negative value is included but omited for the abs() of the value denoting the position.
# TODO: postitions should be a list containing proper locations or improper locations.
def build_include_dict(board_state):
    if len(board_state) == 0:
        return None    
    include = {}
    for word,hot_l in board_state.items():
        for i in range(len(hot_l)):
            hot = hot_l[i]
            if hot == 0:
                continue
            elif hot == 1:
                include[word[i]] = (i+1) * -1
            elif hot == 2:
                include[word[i]] = (i+1)
            else:
                print(f"invalid hot encoding {hot} for {word}")
                exit()
    return include

### main ###
# get the word list
# assumes command is run like $ python3 player.py words.txt
with open(sys.argv[1], 'r') as fd:
    words = fd.read().splitlines()

# board state is a hot encoding where the key is the string, and the value is the encoding like [0,0,1,2,0].
# 0's are misses, 1's are incorrect position, 2's are correct.
board_state = {}
# find letter distribution, receive 5 letters to use
letters = suggest_letters(words)
# suggest a word
word = suggest_word(words, letters)
print(f"play '{word}'")
board_state[word] = [1,1,0,0,1] # update the board state for testing
omit = build_omit_list(board_state)
include = build_include_dict(board_state)
print(omit)
print(include)

#for line in sys.stdin:
    
#    print("What is the board state? Type it like '00120'.")
    
