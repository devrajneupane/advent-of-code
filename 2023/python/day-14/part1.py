tmp = []
pos = open(0).read().splitlines()

for rocks in list(zip(*pos)):
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

tmp = list(zip(*tmp))[::-1]
res = sum(idx * line.count("O") for idx, line in enumerate(tmp, start=1))

print(f"{res = }")
