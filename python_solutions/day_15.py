import re
import functools
import itertools
import collections
from collections import defaultdict
from collections import Counter
import sys
inf = sys.argv[1] if len(sys.argv) > 1 else '../inputs/day_15.txt'

#ll = open(inf).read().strip()
#ll = [[y for y in x] for x in open(inf).read().strip().split('\n\n')]
#ll = [int(x) for x in open(inf).read().strip().split('\n')]
a=open(inf).read().strip().split('\n\n')
ll = [x for x in a[0].split("\n")]
moves = a[1]

def addt(x, y):
    if len(x) == 2:
        return (x[0] + y[0], x[1] + y[1])
    return tuple(map(sum, zip(x, y)))


DIRS = [(0, 1), (0, -1), (1, 0), (-1, 0)]
D = [">", "<", "v", "^"]

moves = [DIRS[D.index(m)] for m in moves.replace("\n", "")]
o = 0

def left(pos):
    return (pos[0], pos[1]-1)
def right(pos):
    return (pos[0], pos[1]+1)

walls = set()
boxes = set()

width = 0
height = 0

for i, l in enumerate(ll):
    for j, ch in enumerate(l):
        j *= 2
        if ch == "#":
            if i > height: height = i
            if j > width: width = j
            walls.add((i,j))
            walls.add((i,j+1))
        if ch == "O":
            boxes.add((i, j))
        if ch == "@":
            robot = (i, j)

def push(box, d):
    assert box in boxes
    nxt = addt(box, d)
    if nxt in walls or right(nxt) in walls:
        return None
    if d[0]:
        # we are moving up/down
        if nxt in boxes:
            r = push(nxt, d)
            if r is None:
                return None
        if left(nxt) in boxes:
            r = push(left(nxt), d)
            if r is None:
                return None
        if right(nxt) in boxes:
            r = push(right(nxt), d)
            if r is None:
                return None
    if d[1] == 1:
        # we are pushing right
        # there can only be one box
        # X>X
        if right(nxt) in boxes:
            r = push(right(nxt), d)
            if r is None:
                return None
    if d[1] == -1:
        # we are pushing left
        # X<X
        if left(nxt) in boxes:
            r = push(left(nxt), d)
            if r is None:
                return None
    boxes.remove(box)
    boxes.add(nxt)
    return True

for move in moves:
    for box in boxes:
        assert right(box) not in boxes
        assert right(box) not in walls
    nxt = addt(robot, move)
    if nxt in walls:
        continue

    if nxt in boxes:
        #print(move)
        copy = {x for x in boxes}
        r = push(nxt, move)
        if r is None:
            boxes = copy
            continue
    elif left(nxt) in boxes:
        #print(move)
        copy = {x for x in boxes}
        r = push(left(nxt), move)
        if r is None:
            boxes = copy
            continue
    assert nxt not in boxes
    assert left(nxt) not in boxes
    robot = nxt

c = 0

grid = [["." if x > 1 else "#" for x in range(width)] + ["#", "#"] if y > 0 else ["#" for x in range(width + 2)] for y in range(height)]
grid.append(["#" for x in range(width + 2)])

for wall in walls:
    grid[wall[0]][wall[1]] = "#"

for box in boxes:
    c += 100 * box[0] + box[1]
    grid[box[0]][box[1]] = "["
    grid[box[0]][box[1]+1] = "]"

grid[robot[0]][robot[1]] = "@"

for line in grid:
    print("".join(line))

print(c)
