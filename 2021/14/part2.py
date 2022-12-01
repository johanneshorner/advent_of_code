from collections import defaultdict

STEPS = 40

with open("input.in") as file:
    template, _, *insertion_rules = file.read().splitlines()
    insertion_rules = dict(rule.split(" -> ") for rule in insertion_rules)

polymers = defaultdict(int, {template[i : i + 2] : 1 for i in range(len(template) - 1)})
elements = defaultdict(int)

for el in template:
    elements[el] += 1

for _ in range(40):
    for polymer, count in polymers.copy().items():
        insert = insertion_rules[polymer]
        polymers[polymer] -= count
        polymers[polymer[0] + insert] += count
        polymers[insert + polymer[1]] += count
        elements[insert] += count

print(max(elements.values()) - min(elements.values()))