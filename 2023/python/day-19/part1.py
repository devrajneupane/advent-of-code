workflows, ratings = open(0).read().split("\n\n")
wf = {}

# someone teach me how to convert this one into dictionary comprehension
for workflow in workflows.splitlines():
    key, rules = workflow[:-1].split("{")
    wf[key] = [rule.split(":") if ":" in rule else rule for rule in rules.split(",")]


def add(rating, workflow="in"):
    result = 0
    exec(rating)
    for rule in wf[workflow]:
        if isinstance(rule, list):
            if eval(rule[0]):
                rule = rule[1]
                break

    if rule == "A":
        return sum(value for value in locals().values() if isinstance(value, int))

    elif rule != "R":
        result = add(rating, rule)
    return result


total = sum(add(rating[1:-1].replace(",", ";")) for rating in ratings.splitlines())
print(total)
