f = open("input.txt","r")

i = ""
for line in f.readlines():
    i += line

caloriesList = []

for elf in i.split("\n\n"):
    calories = 0
    for item in elf.split("\n"):
        if item != "":
            calories += int(item.strip())
    
    for i in range(len(caloriesList)):
        if caloriesList[i] > calories:
            caloriesList.insert(i, calories)
            break
    else:
        caloriesList.append(calories)


print(sum(caloriesList[-3:]))