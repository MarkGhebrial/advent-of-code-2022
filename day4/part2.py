f = open("input.txt","r")

sum = 0

for line in f.readlines():
    assignments = line.strip().split(",")

    for i in range(len(assignments)):
        assignments[i] = assignments[i].split("-")

        for j in range(len(assignments[i])):
            assignments[i][j] = int(assignments[i][j])

    print(assignments)
    if (assignments[0][1] >= assignments[1][0] and assignments[0][0] <= assignments[1][1]) or (assignments[0][1] <= assignments[1][0] and assignments[0][0] >= assignments[1][1]):
        sum += 1
        print("intersect")
    
print(sum)