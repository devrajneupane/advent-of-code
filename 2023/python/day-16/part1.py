layout = open(0).read().splitlines()
xmax, ymax = len(layout), len(layout[0])

moves = {
    "E": {".": {(0, -1,): "E"}, "-": {(0, -1,): "E"}, "|": {(1, 0,): "S", (-1, 0,): "N"}, "\\": {(-1, 0,): "N"}, "/": {(1, 0,): "S"}},
    "W": {".": {(0, 1,): "W"}, "-": {(0, 1): "W"}, "|": {(1, 0,): "S", (-1, 0,): "N"}, "\\": {(1, 0,): "S"}, "/": {(-1, 0,): "N"}},
    "N": {".": {(-1, 0,): "N"}, "|": {(-1, 0): "N"}, "-": {(0, -1,): "E", (0, 1,): "W"}, "\\": {(0, -1,): "E"}, "/": {(0, 1,): "W"}},
    "S": {".": {(1, 0,): "S"}, "|": {(1, 0): "S"}, "-": {(0, -1,): "E", (0, 1,): "W"}, "\\": {(0, 1,): "W"}, "/": {(0, -1,): "E"}},
}
energized, tiles = set(), {(0, 0): "W"}
while tiles:
    (x, y), direction = tiles.popitem()
    while -1 < x < xmax and -1 < y < ymax:
        tile = layout[x][y]
        if (x, y) not in energized or tile in './\\':
            energized.add((x, y))
            move = dict(moves[direction][tile])
            if len(move) > 1:
                (dx, dy), direction = move.popitem()
                tiles[(x + dx, y + dy)] = direction

            (dx, dy), direction = move.popitem()
            x, y = x + dx, y + dy
        else:
            break

print(len(energized))
