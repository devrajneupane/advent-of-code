layout = open(0).read().splitlines()
xmax, ymax = len(layout), len(layout[0])

moves = {
    "E": {".": {(0, -1,): "E"}, "-": {(0, -1,): "E"}, "|": {(1, 0,): "S", (-1, 0,): "N"}, "\\": {(-1, 0,): "N"}, "/": {(1, 0,): "S"}},
    "W": {".": {(0, 1,): "W"}, "-": {(0, 1): "W"}, "|": {(1, 0,): "S", (-1, 0,): "N"}, "\\": {(1, 0,): "S"}, "/": {(-1, 0,): "N"}},
    "N": {".": {(-1, 0,): "N"}, "|": {(-1, 0): "N"}, "-": {(0, -1,): "E", (0, 1,): "W"}, "\\": {(0, -1,): "E"}, "/": {(0, 1,): "W"}},
    "S": {".": {(1, 0,): "S"}, "|": {(1, 0): "S"}, "-": {(0, -1,): "E", (0, 1,): "W"}, "\\": {(0, 1,): "W"}, "/": {(0, -1,): "E"}},
}


def cnt(tiles):
    energized = set()
    while tiles:
        (x, y), direction = tiles.popitem()
        while -1 < x < xmax and -1 < y < ymax:
            tile = layout[x][y]
            if (x, y) not in energized or tile in "./\\":
                energized.add((x, y))
                move = dict(moves[direction][tile])
                if len(move) > 1:
                    (dx, dy), direction = move.popitem()
                    tiles[(x + dx, y + dy)] = direction

                (dx, dy), direction = move.popitem()
                x, y = x + dx, y + dy
            else:
                break
    return len(energized)


res = 0

for x, y in zip([*range(xmax)], [*range(ymax)]):
    res = max(cnt({(x, 0): "W"}), cnt({(x, ymax-1): "E"}), cnt({(0, y): "S"}), cnt({(xmax-1, y): "N"}), res)

print(res)
