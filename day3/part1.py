f = open("input.txt","r")

def priority(c):
    if ord(c) in range(ord('a'), ord('z') + 1):
        return ord(c) - ord('a') + 1
    else:
        return ord(c) - ord('A') + 27

sum = 0

for rucksack in f.readlines():
    rucksack = rucksack.strip()

    mid = int(len(rucksack)/2)
    firstHalf = rucksack[:mid]
    secondHalf = rucksack[mid:]

    for c in firstHalf:
        if c in secondHalf:
            #print(rucksack)
            sum += priority(c)
            break

print(sum)