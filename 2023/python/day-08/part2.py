from itertools import cycle
from math import lcm

data = open(0).read().split("\n\n")

places, lengths = {}, []
lengths = []
direction = cycle(data[0])

for place in data[1].splitlines():
    place = place.split(" = ")
    place[1] = (place[1][1:4], place[1][6:9])
    places[place[0]] = place[1]

cur_pos = [place for place in places if place.endswith("A")]

for pos in cur_pos:
    tmp = 0
    while not pos.endswith("Z"):
        pos = places[pos][0] if next(direction) == "L" else places[pos][1]
        tmp += 1

    lengths.append(tmp)

print(lcm(*lengths))

# i think following loop probably find the answer but i can't wait for entire december :)
# while not all(pos.endswith("Z") for pos in cur_pos):
#     d = next(direction)
#     cur_pos = [places[pos][0] if d == "L" else places[pos][1] for pos in cur_pos]
#     steps += 1
