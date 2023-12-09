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

    history.append(0)
    sequences.reverse()
    for i, sequence in enumerate(sequences[1:]):
        sequence.append(sequence[-1] + sequences[i][-1])

    res += sequences[-1][-1]

print(f"{res = }")
