import re

res = 0
data = open(0).read().splitlines()
max_idx = len(data) - 1
for idx, line in enumerate(data):
    gears = re.finditer(r"\*", line)
    for gear in gears:
        nums = []
        start_idx, end_idx = gear.start(), gear.end() + 3
        pos = range(start_idx - 1, start_idx + 2)
        dx = 3 if start_idx else 0
        dy = [-1, 0, 1] if max_idx > idx > 0 else [0, 1] if not idx else [-1, 0]
        offset = start_idx - dx
        for y in dy:
            frame = data[idx + y][offset:end_idx]
            gear_numbers = re.finditer(r"\d+", frame)
            for gear_number in gear_numbers:
                g_start_idx = gear_number.start() + offset
                g_end_idx = gear_number.end() + offset - 1
                if g_start_idx in pos or g_end_idx in pos:
                    nums.append(gear_number.group())
        if len(nums) == 2:
            res += int(nums[0]) * int(nums[1])


print(res)
