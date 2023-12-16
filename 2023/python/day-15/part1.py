def hash(s):
    val = 0
    for ch in s:
        val = ((val + ord(ch)) * 17) % 256
    return val


res = sum(hash(s) for s in input().split(","))
print(res)
