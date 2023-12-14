view, idx = [], 0
pos = open(0).read().splitlines()


def spin(pos):
    tmp = []
    for rocks in pos:
        cnt, lst = 0, []
        for rock in rocks:
            if rock == ".":
                cnt += 1
                continue
            elif rock == "#":
                lst.extend(["."] * cnt)
                cnt = 0
            lst.append(rock)

        if length := len(pos[0]) - len(lst):
            lst += ["."] * length

        tmp.append(lst)

    return tmp


while True:
    pos = spin(list(zip(*pos)))  # North
    pos = spin(list(zip(*pos)))  # West
    pos = spin([line[::-1] for line in zip(*pos)])  # South
    pos = spin([line[::-1] for line in zip(*[line[::-1] for line in pos])])  # East

    pos = ["".join(line[::-1]) for line in pos]
    idx += 1

    if pos in view:
        break

    view.append(pos)

first = view.index(pos)
rocks = view[(1000000000 - first) % (idx - first) + first]
res = sum(idx * line.count("O") for idx, line in enumerate(rocks[::-1], start=1))

print(f"{res = }")
