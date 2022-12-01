with open("input.in") as file:
    input = file.read()
    width = input.find("\n")
    heightmap_flattened = [int(height) for height in input.replace("\n", "")]

def is_low_point(idx: int) -> bool:
    #left bound check
    if idx % width != 0:
        if heightmap_flattened[idx - 1] <= heightmap_flattened[idx]: return False
    #right bound check
    if (idx + 1) % width != 0:
        if heightmap_flattened[idx + 1] <= heightmap_flattened[idx]: return False
    #upper bound check
    if idx - width >= 0:
        if heightmap_flattened[idx - width] <= heightmap_flattened[idx]: return False
    #lower bound check
    if idx + width < len(heightmap_flattened):
        if heightmap_flattened[idx + width] <= heightmap_flattened[idx]: return False
    return True

print(sum(heightmap_flattened[idx] + 1 for idx in range(len(heightmap_flattened)) if is_low_point(idx)))

print([idx for idx in range(len(heightmap_flattened)) if is_low_point(idx)])
