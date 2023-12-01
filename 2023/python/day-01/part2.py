data = open(0).read()
sum = 0
spelled_digits = {
    "one": 'o1e',
    "two": 't2o',
    "three": 't3e',
    "four": 'f4r',
    "five": 'f5e',
    "six": 's6x',
    "seven": 's7n',
    "eight": 'e8t',
    "nine": 'n9e',
}
for key, value in spelled_digits.items():
    if key in data:
        data = data.replace(key, value)

for line in data.splitlines():
    digits = [char for char in line if char.isdigit()]
    sum += int(digits[0]+digits[-1])

print(sum)
