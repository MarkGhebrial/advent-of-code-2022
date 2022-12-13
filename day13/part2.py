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
    
packetList = [[[2]], [[6]]]
for packet in i.split("\n"):
    if packet == "": continue

    packetList.append(eval(packet.strip()))

def sortPackets():
    for i in range(len(packetList)):
        for j in range(i, len(packetList)):
            if compare(packetList[j], packetList[i]) == True:
                temp = packetList[i]
                packetList[i] = packetList[j]
                packetList[j] = temp
        
sortPackets()
print(packetList)

product = 1
for i in range(len(packetList)):
    if packetList[i] == [[2]] or packetList[i] == [[6]]:
        product *= i + 1

print(product)