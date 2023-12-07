res = 1
data = [line.split()[1:] for line in open(0).read().splitlines()]
data = [ int(''.join(item)) for item in data ]

# oc i forget about quadratic equations, it just took a sip of tea to execute so i'm leaving it
res *= len([d for s in range(1,data[0]) if (d := s *(data[0]-s)) > data[1]])

print(res)


