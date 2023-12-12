from itertools import combinations

image = open(0).read().splitlines()
x_offset, y_offset = 0, 0

tmp_x = {
    x: (x_offset := x_offset + 1) if len(set(line)) == 1 else x_offset
    for x, line in enumerate(image)
}

# transpose image
image = ["".join(line) for line in zip(*image)]

tmp_y = {
    y: (y_offset := y_offset + 1) if len(set(line)) == 1 else y_offset
    for y, line in enumerate(image)
}

# transpose image back to original form
image = ["".join(line) for line in zip(*image)]

# coordinates of expanded galaxies
galaxies = [
    (x + tmp_x[x] * 999999, y + tmp_y[y] * 999999)
    for x, line in enumerate(image)
    for y, galaxy in enumerate(line)
    if galaxy == "#"
]
res = sum(abs(x0 - x1) + abs(y0 - y1) for (x0, y0), (x1, y1) in combinations(galaxies, 2))

print(res)
