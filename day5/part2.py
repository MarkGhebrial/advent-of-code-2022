stackFile = open("stacks.txt","r")
input = open("input.txt","r")

levels = stackFile.readlines()
stacks = [[], [], [], [], [], [], [], [], []]

for i in reversed(range(0, 8)): # Iterate thru each stack level
    # Iterate thru each stack
    stack = 0
    for j in range(1, 34, 4):
        if levels[i][j] != " ":
            stacks[stack].append(levels[i][j])
        stack += 1

print(stacks)

for line in input.readlines():
    line = line.strip().split(" ")

    qty = int(line[1])
    origin = int(line[3])
    destination = int(line[5])

    stacks[destination - 1] += stacks[origin - 1][-qty:]

    for i in range(qty):
        stacks[origin - 1].pop()

print(stacks)

out = ""
for stack in stacks:
    out += stack[-1]

print(out)