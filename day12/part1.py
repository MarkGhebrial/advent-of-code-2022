from queue import *

f = open("input.txt","r")

heightmap = []

start = (0, 0)
end = (0, 0)

row_no = 0
col_no = 0
for line in f.readlines():
    heightmap.append([])
    for c in line.strip():
        if c == "S":
            start = (row_no, col_no)
            heightmap[row_no].append(-1)
        elif c == "E":
            end = (row_no, col_no)
            heightmap[row_no].append(ord('z') + 1)
        else:
            heightmap[row_no].append(ord(c))

        col_no += 1

    col_no = 0
    row_no += 1

class TreeNode:
    def __init__(self, value, parent = None):
        self.parent = parent
        self.value = value

def min_dist_to_end(start, map):
    q = Queue()

    root = TreeNode(start)
    visited = [start]
    q.put(root)

    while not q.empty():
        node = q.get()
        v = node.value

        print("Popped", v)

        #if map[v[0]][v[1]] == -2:
        #    return "We found the end!"
        
        for edge in [(v[0] + 1, v[1]), (v[0] - 1, v[1]), (v[0], v[1] + 1), (v[0], v[1] - 1)]:
            print("Iterating on", edge)

            if edge[0] < 0 or edge[1] < 0 or edge[0] >= len(map) or edge[1] >= len(map[0]):
                print("Out of bounds")
                continue

            if map[edge[0]][edge[1]] == ord('z') + 1 and map[v[0]][v[1]] == ord('y'):
                count = 0
                node = TreeNode(edge, node)
                while not node.parent == None:
                    print(node.value)
                    count += 1
                    node = node.parent
                return count

            if (not edge in visited) and (map[v[0]][v[1]] == -1 or map[edge[0]][edge[1]] - map[v[0]][v[1]] <= 1):
                print("Putting", edge)
                visited.append(edge)
                q.put(TreeNode(edge, node))

        print("")

    print("Not found!")

    for i in range(len(map)):
        line = ""
        for j in range(len(map[0])):
            if (i, j) not in visited:
                line += (chr(map[i][j]))
            else:
                line += " "
        print(line)


dist = min_dist_to_end(start, heightmap)
print(dist)