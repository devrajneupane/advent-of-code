sum = 0
for line in open(0).read().splitlines():
    digits = [ch for ch in line if ch.isdigit()]
    sum += int(digits[0]+digits[-1])

print(sum)
