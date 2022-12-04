f = open("input.txt","r")

sum = 0

for line in f.readlines():
    line = line.strip()
    s = line.split(" ")
    theirMove = s[0]
    ourMove = s[1]

    score = 0
    if ourMove == "X":
        score += 1
    elif ourMove == "Y":
        score += 2
    elif ourMove == "Z":
        score += 3

    if (theirMove == "A" and ourMove == "X") or (theirMove == "B" and ourMove == "Y") or (theirMove == "C" and ourMove == "Z"):
        score += 3
        print("tied")
    elif (theirMove == "A" and ourMove == "Y") or (theirMove == "B" and ourMove == "Z") or (theirMove == "C" and ourMove == "X"):
        print("won")
        score += 6
    else:
        print("lost")

    sum += score

print(sum)