import sys,csv

with open(sys.argv[1],newline='') as csvfile:
    reader = csv.reader(csvfile, delimiter=',')
    acc = 0
    count = 0
    failed = 0
    for row in reader:
        guesses = int(row[1])
        if guesses <= 6:
            count += 1
            acc += guesses
        else:
            failed += 1
    print(f"average guess is {acc/count} with {failed} failing")
