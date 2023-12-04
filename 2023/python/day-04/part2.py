data = open(0).read().splitlines()

record = {i: 1 for i in range(1, len(data) + 1)}
for i, line in enumerate(data, start=1):
    line = line.split(":")[-1].split("|")
    for idx in range(len(line)):
        line[idx] = [i for i in line[idx].strip().split(" ") if i != ""]
    match_count = len(set(line[0]) & set(line[1]))
    for _ in range(record[i]):
        for c in range(1, match_count + 1):
            record[i + c] += 1


print(sum(record.values()))
