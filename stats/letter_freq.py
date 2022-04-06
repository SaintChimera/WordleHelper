import sys


with open(sys.argv[1], 'r') as fd:
    words = fd.readlines()

letter_freq = {}
for word in words:
    word = word.strip("\n")
    for letter in word:
        if letter in letter_freq:
            letter_freq[letter] += 1
        else:
            letter_freq[letter] = 1

sorted_letter_freq = dict(sorted(letter_freq.items(),key=lambda x:x[1],reverse=True))
#print(sorted_letter_freq)

counter = 0
acc1 = 0
acc2 = 0
for key,value in sorted_letter_freq.items():
    if counter < 10:
        acc1 += value
    acc2 += value
    counter += 1

print(acc1/acc2)
