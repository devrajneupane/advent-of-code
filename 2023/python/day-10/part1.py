pipe_map = {
    "-": ((0, 1), (0, -1)),
    "|": ((1, 0), (-1, 0)),
    "F": ((0, 1), (1, 0)),
    "7": ((0, -1), (1, 0)),
    "L": ((0, 1), (-1, 0)),
    "J": ((0, -1), (-1, 0)),
    "S": ((0, 1), (1, 0), (0, -1), (-1, 0)),
    " ": (),
}
cnt = 0
pipes = open(0).read().splitlines()
xmax, ymax = len(pipes), len(pipes[0])
x, y = next((i, row.index("S")) for i, row in enumerate(pipes) if "S" in row)


def get_pipes(x, y, ne):
    return [
        (a, b,)
        for dx, dy in ne
        if 0 <= (a := x + dx) < xmax and 0 <= (b := y + dy) < ymax
    ]


px, py = -1, -1  # some random value at which 'S' is not present
while pipes[px][py] != "S" or cnt < 3:
    for nx, ny in get_pipes(x, y, pipe_map[pipes[x][y]]):
        pipe = get_pipes(nx, ny, pipe_map[pipes[nx][ny]])
        if (x, y) in pipe and (nx, ny) != (px, py):
            px, py, x, y = x, y, nx, ny
            cnt += 1
            break

print(cnt // 2)
