from itertools import cycle

data = open(0).read().split("\n\n")
steps, places = 0, {}
direction = cycle(data[0])

for place in data[1].splitlines():
    place = place.split(" = ")
    place[1] = (place[1][1:4], place[1][6:9])
    places[place[0]] = place[1]

cur_pos = "AAA"
while cur_pos != "ZZZ":
    cur_pos = places[cur_pos][0] if next(direction) == "L" else places[cur_pos][1]
    steps += 1

print(steps)
