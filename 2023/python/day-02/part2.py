from operator import mul
from functools import reduce

res = 0
for line in open(0).read().splitlines():
    line = line.split(":")
    game_id = int(line[0].split(" ")[-1])
    tmp = {"red": 0, "green": 0, "blue": 0}
    for subset in line[1].split(";"):
        for cube in subset.split(","):
            values = cube.split(" ")
            thres, color = values[1:]
            if int(thres) > tmp[color]:
                tmp[color] = int(thres)
    res += reduce(mul, list(tmp.values()))

print(res)
