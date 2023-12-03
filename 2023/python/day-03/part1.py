import re
import string

data = open(0).read().splitlines()

res = 0
max_idx = len(data) - 1
symbols = set(string.punctuation) - set(".")
for idx, line in enumerate(data):
    matches = re.finditer(r"\d+", line)
    for match in matches:
        start_idx, end_idx = match.start(), match.end() + 1
        dx = 1 if start_idx else 0
        dy = [-1, 0, 1] if max_idx > idx > 0 else [0, 1] if not idx else [-1, 0]
        for y in dy:
            if set(data[idx + y][start_idx - dx : end_idx]) & symbols:
                res += int(match.group())

print(res)
