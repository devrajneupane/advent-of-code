import re

res = 0
report = [
    [int(value) for value in re.findall(r"[-]?\d+", history)]
    for history in open(0).read().splitlines()
]

for history in report:
    sequences = [history]

    while any(history):
        sequences.append([value - history[i] for i, value in enumerate(history[1:])])
        history = sequences[-1]

    history.insert(0, 0)
    sequences.reverse()
    for i, sequence in enumerate(sequences[1:], start=1):
        sequence.insert(0, sequence[0] - sequences[i - 1][0])

    res += sequences[-1][0]

print(f"{res = }")
