f = open("test.txt","r")

monkeys = {}
for line in f.readlines():
    line = line.strip().split(" ")
    print(line)

    monkeys[line[0][:-1]] = "(" + " ".join(line[1:]) + ")"

print(monkeys)

expr = monkeys["root"]

print(expr)

while True:
    try:
        print(eval(expr))
        break

    except:
        for monkey in monkeys:
            if monkey in expr:
                expr = expr.replace(monkey, monkeys[monkey])
        print(expr)
