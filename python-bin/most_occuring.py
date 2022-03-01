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

### main ###
# get the word list
# assumes command is run like $ python3 player.py words.txt
with open(sys.argv[1], 'r') as fd:
    words = fd.read().splitlines()

frequent = frequent_letters(words)

for key, value in frequent.items():
    print(key,value)
