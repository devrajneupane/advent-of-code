res = 1
total_ways = []

data = [line.split()[1:] for line in open(0).read().splitlines()]
data = [[int(item) for item in record] for record in data]

for i, time in enumerate(data[0]):
    num_ways = [d for s in range(1,time) if (d := s *(time-s)) > data[1][i]]
    total_ways.append(num_ways)

for i in total_ways:
    res *= len(i)

print(res)
