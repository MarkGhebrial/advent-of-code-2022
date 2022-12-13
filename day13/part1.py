f = open("input.txt","r")

i = ""
for line in f.readlines():
    i += line

def compare(left, right):
    print(left, right)

    if type(left) != type([]):
        left = [left]
    if type(right) != type([]):
        right = [right]

    if len(left) == 1 and len(right) == 1:
        try:
            if left[0] < right[0]:
                return True
            if left[0] > right[0]:
                return False
            return None
        except:
            print("Uh oh")

    for i in range(0, min(len(left), len(right))):
        #print("Comparing:", left[i], right[i])
        comparison = compare(left[i], right[i])
        if comparison == None:
            continue
        return comparison

    if len(left) < len(right):
        return True
    elif len(left) > len(right):
        return False
    else:
        return None
    
sum = 0
index = 0
for pair in i.split("\n\n"):
    pair = pair.split("\n")
    left = eval(pair[0].strip())
    right = eval(pair[1].strip())

    index += 1
    if compare(left, right):
        sum += index
        print(index)

print(sum)