import os, sys

# open the wordlist and push 5 letter words to a new list
with open(sys.argv[1],'r') as fd_in:
    words = fd_in.readlines()
for word in words:
    word = word.strip("\n")
    with open(sys.argv[2],'a') as fd_out:
        if len(word) == 5:
            print(word)
            fd_out.write(word+"\n")
