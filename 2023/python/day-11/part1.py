from itertools import combinations

tmp = []
for line in open(0).read().splitlines():
    if len(set(line)) == 1:
        tmp += [line] * 2
    else:
        tmp.append(line)

# transpose image
tmp = ["".join(line) for line in zip(*tmp)]

image = []
for line in tmp:
    if len(set(line)) == 1:
        image += [line] * 2
    else:
        image.append(line)

# transpose image back to original form
image = ["".join(line) for line in zip(*image)]

# coordinates of expanded galaxies
galaxies = [
    (x, y,)
    for x, line in enumerate(image)
    for y, galaxy in enumerate(line)
    if galaxy == "#"
]
res = sum(abs(x0 - x1) + abs(y0 - y1) for (x0, y0), (x1, y1) in combinations(galaxies, 2))

print(res)
