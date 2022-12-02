f = open("input.txt","r")

i = ""
for line in f.readlines():
    i += line

maxCalories = 0

for elf in i.split("\n\n"):
    calories = 0
    for item in elf.split("\n"):
        if item != "":
            calories += int(item.strip())
    
    if calories > maxCalories:
        maxCalories = calories


print(maxCalories)