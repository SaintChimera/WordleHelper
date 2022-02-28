import os, sys
from itertools import product
from collections import OrderedDict

# function to build an omit list with the board state.
def _build_omit_list(board_state):
    omit = []
    if len(board_state) != 0:
        for word,hot_l in board_state.items():
            for i in range(len(hot_l)):
                hot = hot_l[i]
                if hot == '0':
                    omit.append(word[i])
                elif hot == '1':
                    continue
                elif hot == '2':
                    continue
                else:
                    print(f"invalid hot encoding {hot} for {word}")
                    exit()
    return omit

# function to build an include list with the board list
# include dict has a key of the letter, value of the desired position. a negative value is included but omited for the abs() of the value denoting the position.
def _build_include_dict(board_state):
    include = {}
    if len(board_state) != 0:
        for word,hot_l in board_state.items():
            for i in range(len(hot_l)):
                letter = word[i]
                hot = hot_l[i]
                if hot == '0':
                    continue
                elif hot == '1':
                    position = (i+1) * -1
                    if letter in include:
                        letter_list = include[letter]
                        letter_list.append(position)
                        include[letter] = letter_list
                    else:
                        include[letter] = [position]
                elif hot == '2':
                    position = i+1
                    if letter in include:
                        letter_list = include[letter]
                        letter_list.append(position)
                        include[letter] = letter_list
                    else:
                        include[letter] = [position]
                else:
                    print(f"invalid hot encoding {hot} for {word}")
                    exit()
    return include

# function to find the most common 5 letters in the word list.
def _suggest_letters(words, omit=None, include={}):
    # make a dict with the number of occurences for ever letter.
    letter_dist = {}
    for word in words:
        for letter in word:
            if letter in letter_dist:
                letter_dist[letter] += 1
            else:
                letter_dist[letter] = 1
    
    # return the top 5 letters at a time
    letters = include.keys() # has to include these letters
    letters_string = ''.join(a for a in letters)
    # remove include letters from letters_dict since we're going to be building new sets from letters_dict
    for letter in letters:
        letter_dist.pop(letter)
    # remove omit letters from letters_dist
    for letter in omit:
        letter_dist.pop(letter)
    # get letters
    letter_dist_sorted = sorted(letter_dist, key=letter_dist.get, reverse=True)
    iter_size = 5-len(letters)
    for it in product(letter_dist_sorted, repeat=iter_size):
        # check that we have 5 unique letters
        dedup_set = set()
        for a in it:
            dedup_set.add(a)
        if len(dedup_set) < iter_size: # try again if we do
            continue
        else: #return if we dont
            yield_str = letters_string + ''.join(a for a in it)
            yield yield_str

# finds all possible words according to the letters list, not considering the include list
def _find_words(words, letters):
    suggestable_words = []
    for word in words:
        if all(letter in word for letter in letters):
            suggestable_words.append(word)
    return suggestable_words

def _get_suggested_word(suggestable_words, include):
    suggested_word = ''
    if len(include) == 0: # there is no include list
        suggested_word = suggestable_words[0]
        return suggested_word
    # find word with proper include positions and improper include positions
    all_sugg = []
    for suggestable_word in suggestable_words:
        valid_word = True
        for letter,positions in include.items():
            # first check if an include letter is in our suggestable_word. because it has to be
            suggestable_word_letter_loc = suggestable_word.find(letter)
            if suggestable_word_letter_loc < 0:
                valid_word = False
                break
            # if the letter is a proper location letter and the letter was not found in that proper location.
            for position in positions:
                if position > 0:
                    if position-1 != suggestable_word_letter_loc:
                        valid_word = False
                        break
                # if the letter is an improper location letter and the letter was found in the same location.
                if position < 0:
                    if (position*-1)-1 == suggestable_word_letter_loc:
                        valid_word = False
                        break
        if valid_word:
            all_sugg.append(suggestable_word)
    if len(all_sugg) > 0:
        print(all_sugg)
        return all_sugg[0]
    else:
        return suggested_word

# function to suggest a word according to the board state and the 5 most common letters in the word list.
def suggest_word(words, board_state):
    # build omit list and include dict from board state
    omit = _build_omit_list(board_state)
    include = _build_include_dict(board_state)

    for letters in _suggest_letters(words, omit, include):
        suggestable_words = _find_words(words, letters)
        suggested_word = _get_suggested_word(suggestable_words, include)
        # check if we actually got a word.
        if suggested_word != '':
            return suggested_word
        
    return ''


### main ###
# get the word list
# assumes command is run like $ python3 player.py words.txt
with open(sys.argv[1], 'r') as fd:
    words = fd.read().splitlines()

# board state is a hot encoding where the key is the string, and the value is the encoding like [0,0,1,2,0].
# 0's are misses, 1's are incorrect position, 2's are correct.
board_state = {}
# suggest a word
word = suggest_word(words, board_state)
print(f"play '{word}'")

line = ''
while line != ['2','2','2','2','2']:
    line = input("What is the board state? Type it like '00120'.")
    # update board state
    line = list(line.strip('\n'))
    board_state[word] = line # update the board state for testing
    
    # get another word
    word = suggest_word(words, board_state)
    print(f"play '{word}'")

