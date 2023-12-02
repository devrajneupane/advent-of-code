res = 0
cubes = {"red": 12, "green": 13, "blue": 14}
for line in open(0).read().splitlines():
    line = line.split(":")
    game_id = int(line[0].split(" ")[-1])
    flag = False
    for subset in line[1].split(";"):
        for cube in subset.split(","):
            values = cube.split(" ")
            if int(values[-2]) > cubes[values[-1]]:
                flag = True
                break
        if flag:
            break
    if not flag:
        res += game_id

print(res)
