with open("input.in") as file:
    heightmap = [[int(height) for height in line.strip('\n') ] for line in file]
    
for row in heightmap:
    row.insert(0, 9)
    row.append(9)

heightmap.insert(0, [9] * len(heightmap[0]))
heightmap.append([9] * len(heightmap[0]))

def is_low_point(x: int, y: int) -> bool:
    if heightmap[y][x - 1] <= heightmap[y][x]: return False
    if heightmap[y][x + 1] <= heightmap[y][x]: return False
    if heightmap[y - 1][x] <= heightmap[y][x]: return False
    if heightmap[y + 1][x] <= heightmap[y][x]: return False
    return True

print(sum(heightmap[y][x] + 1 for x in range(len(heightmap[0])) for y in range(len(heightmap)) if is_low_point(x, y)))