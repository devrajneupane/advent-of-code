coords = [(0, 0)]
dirs = {"R": (0, 1), "D": (1, 0), "L": (0, -1), "U": (-1, 0)}
boundary_points = 0

for line in open(0):
    direction, step = line.split()[:-1]
    step = int(step)
    dx, dy = dirs[direction]
    x, y = coords[-1]
    coords.append((x + dx * step, y + dy * step))
    boundary_points += step

# Shoelace formula
area = 0.5 * abs(sum( x * (coords[(i + 1) % len(coords)][1] - coords[i - 1][1]) for i, (x, _) in enumerate(coords) ))

# Pick's theorem
interior_points = area - boundary_points // 2 + 1

print(interior_points+boundary_points)
