def parse_input(input_str: str):
    template = input_str.split("\n\n")[0]
    insertion_rule_lines = input_str.split("\n\n")[1].split("\n")

    return (
        template,
        {left : right for left, right in (insertion_rule.split(" -> ") for insertion_rule in insertion_rule_lines)}
    )

with open("input.in") as file:
    template, insertion_rules = parse_input(file.read())

def apply_step(template: str, insertion_rules):
    return "".join([template[i] + insertion_rules[template[i : i + 2]] for i in range(len(template) - 1)]) + template[-1]

polymer = template
for i in range(10):
    polymer = apply_step(polymer, insertion_rules)

occurences = {char: 0 for char in set(polymer)}

for char in occurences:
    occurences[char] += polymer.count(char)

print(max(occurences.values()) - min(occurences.values()))