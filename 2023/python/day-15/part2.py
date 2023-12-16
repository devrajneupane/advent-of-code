def hash(s):
    val = 0
    for ch in s:
        val = ((val + ord(ch)) * 17) % 256
    return val


boxes = [[] for _ in range(256)]
res, focal_lenghts = 0, {}

for s in input().split(","):
    if "=" in s:
        label, focal_length = s.split("=")
        box = hash(label)
        if label not in boxes[box]:
            boxes[box].append(label)
        focal_lenghts[label] = focal_length
    else:
        label = s[:-1]
        box = hash(label)
        if label in boxes[box]:
            boxes[box].remove(label)

for idx, box in enumerate(boxes, start=1):
    for slot, label in enumerate(box, start=1):
        res += idx * slot * int(focal_lenghts[label])
print(res)
