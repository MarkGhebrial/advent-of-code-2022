from sympy import *

f = open("input.txt","r")

monkeys = {}
for line in f.readlines():
    line = line.strip().split(" ")

    if line[0][:-1] == "humn":
        continue

    if line[0][:-1] == "root":
        monkeys["root"] = "(" + line[1] + " - " + line[3] + ")"
        continue

    monkeys[line[0][:-1]] = "(" + " ".join(line[1:]) + ")"

#print(monkeys)

expr = monkeys["root"]

#print(expr)

while True:
    for monkey in monkeys:
        if monkey in expr:
            #print(monkey)
            expr = expr.replace(monkey, monkeys[monkey])
            break
    else:
        break

print(sympify(expr))
print("humn =", solveset(expr))
