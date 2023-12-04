res = 0

for line in open(0).read().splitlines():
    line = line.split(":")[-1].split("|")
    for idx in range(len(line)):
        line[idx] = [i for i in line[idx].strip().split(" ") if i != '']
    if points := len(set(line[0]) & set(line[1])):
        res += 2 ** (points - 1)

print(res)
