def count(springs, size):
    cnt = 0

    if not springs:
        return 1 if not size else 0

    if not size:
        return 0 if "#" in springs else 1

    # assuming ? as .
    if springs[0] in ".?":
        cnt += count(springs[1:], size)

    # assuming ? as #
    if springs[0] in ("#?"):
        if (
            len(springs) >= size[0]
            and "." not in springs[: size[0]]
            and (len(springs) == size[0] or springs[size[0]] != "#")
        ):
            cnt += count(springs[size[0] + 1 :], size[1:])

    return cnt


res = 0
for row in open(0).read().splitlines():
    springs, size = row.split()
    size = [int(i) for i in size.split(",")]
    res += count(springs, size)

print(f"{res = }")
