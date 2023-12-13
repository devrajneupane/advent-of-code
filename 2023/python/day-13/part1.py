def reflection(pattern):
    for idx in range(1, len(pattern)):
        above = pattern[:idx][::-1]
        below = pattern[idx:]

        above = above[: len(below)]
        below = below[: len(above)]

        if above == below:
            return idx
    return 0


res = 0
for pattern in open(0).read().split("\n\n"):
    pattern = pattern.splitlines()
    if idx := reflection(pattern):
        res += idx * 100
    else:
        pattern = ["".join(line) for line in zip(*pattern)]
        res += reflection(pattern)

print(res)
