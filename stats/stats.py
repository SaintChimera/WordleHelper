import sys,csv

with open(sys.argv[1],newline='') as csvfile:
    reader = csv.reader(csvfile, delimiter=',')
    acc = 0
    count = 0
    failed = 0
    two_count = 0
    three_count = 0
    four_count = 0
    five_count = 0
    six_count = 0
    for row in reader:
        guesses = int(row[1])
        if guesses <= 6:
            count += 1
            acc += guesses
            if guesses == 2:
                two_count += 1
            if guesses == 3:
                three_count += 1
            if guesses == 4:
                four_count += 1
            if guesses == 5:
                five_count += 1
            if guesses == 6:
                six_count += 1
        else:
            failed += 1
    print(f"average guess is {acc/count} with {failed} failing. 2:{two_count}, 3:{three_count}, 4:{four_count}, 5:{five_count}, 6:{six_count}")
