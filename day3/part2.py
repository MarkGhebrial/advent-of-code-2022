import itertools

f = open("input.txt","r")

def priority(c):
    if ord(c) in range(ord('a'), ord('z') + 1):
        return ord(c) - ord('a') + 1
    else:
        return ord(c) - ord('A') + 27

rucksacks = f.readlines()


def groups():
    currGroup = 0
    index = 0
    while index + 3 <= len(rucksacks):
        group = rucksacks[index:index+3]

        for i in group:
            i = i.strip()
        
        currGroup += 1
        index = currGroup * 3

        yield group

sum = 0

for group in groups():
    print(group)

    #for rucksack in group:
        #mid = int(len(rucksack)/2)
        #firstHalf = rucksack[:mid]
        #secondHalf = rucksack[mid:]

    for c in group[0]:
        if c in group[1] and c in group[2]:
            sum += priority(c)
            break

print(sum)