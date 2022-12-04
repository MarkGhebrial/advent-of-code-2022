f = open("input.txt","r")

sum = 0

secretSauce = {
    "A": { # Their rock
        "X": "scissors", # lose
        "Y": "rock",     # draw
        "Z": "paper",    # win
    },
    "B": { # paper
        "X": "rock",
        "Y": "paper",
        "Z": "scissors",
    },
    "C": { # scissors
        "X": "paper",
        "Y": "scissors",
        "Z": "rock",
    },
}

for line in f.readlines():
    line = line.strip()
    s = line.split(" ")
    theirMove = s[0]
    desiredResult = s[1]

    ourMove = secretSauce[theirMove][desiredResult]

    score = 0
    if ourMove == "rock":
        score += 1
    elif ourMove == "paper":
        score += 2
    elif ourMove == "scissors":
        score += 3

    if (theirMove == "A" and ourMove == "rock") or (theirMove == "B" and ourMove == "paper") or (theirMove == "C" and ourMove == "scissors"):
        score += 3
        print("tied")
    elif (theirMove == "A" and ourMove == "paper") or (theirMove == "B" and ourMove == "scissors") or (theirMove == "C" and ourMove == "rock"):
        print("won")
        score += 6
    else:
        print("lost")

    sum += score

print(sum)