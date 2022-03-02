import os, sys

def frequent_letters(words):
    letter_dist = {}
    for word in words:
        for i in range(len(word)):
            letter = word[i]
            if letter in letter_dist:
                letter_l = letter_dist[letter]
                letter_l[i] += 1
                letter_dist[letter] = letter_l
            else:
                letter_l = [0]*5
                letter_l[i] = 1
                letter_dist[letter] = letter_l
    return letter_dist

# sort first row by largest first.
def sort_largest(frequent, position):
    ret_row = {}
    for i in range(len(frequent)):
        largest_key = ''
        largest_value = 0
        for key, value in frequent.items():
            if value[0] > largest_value and key not in ret_row:
                largest_key = key
                largest_value = value[0]
        ret_row[largest_key] = largest_value
    return ret_row

### main ###
# get the word list
# assumes command is run like $ python3 player.py words.txt
with open(sys.argv[1], 'r') as fd:
    words = fd.read().splitlines()

frequent = frequent_letters(words)

first_row = sort_largest(frequent, 0)
second_row = sort_largest(frequent, 1)
third_row = sort_largest(frequent, 2)
fourth_row = sort_largest(frequent, 3)
fifth_row = sort_largest(frequent, 4)

