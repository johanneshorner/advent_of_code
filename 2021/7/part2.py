input = open("input.in", "r").read()

crab_horizontal_positions = [int(pos) for pos in input.split(',')]

fuel_usages = [sum(map(lambda crab_pos: sum(range(1, abs(crab_pos - pos) + 1, 1)), crab_horizontal_positions)) for pos in range(max(crab_horizontal_positions))]

print(min(fuel_usages))