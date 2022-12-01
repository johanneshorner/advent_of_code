import re
import itertools

def get_input():
    with open('input.in', 'r') as file:
        cuboids = [[
                [int(value) for value in string.split("..")] if ".." in string
                 else string for string in re.findall("on|off|-?\d{1,}\.\.-?\d{1,}", line.strip())]  for line in file]
        return cuboids

def set_cubes(cuboids: list[list[str]]):
    on_cubes = set()

    for action, (x1, x2), (y1, y2), (z1, z2) in cuboids:
        if any(val < -50 for val in [x1, y1, z1]) or any(val > 50 for val in [x2, y2, z2]):
            continue
        cubes = list(itertools.product(range(x1, x2 + 1), range(y1, y2 + 1), range(z1, z2 + 1)))

        for cube in cubes:
            if action == "on":
                on_cubes.add(cube)
            else:
                if cube in on_cubes:
                    on_cubes.remove(cube)

    return len(on_cubes)

cuboids = get_input()

print(set_cubes(cuboids))