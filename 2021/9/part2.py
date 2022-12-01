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

low_points = [(x,y) for x in range(len(heightmap[0])) for y in range(len(heightmap)) if is_low_point(x, y)]

from enum import Enum
class PreviousLocation(Enum):
    NONE = 0
    UP = 1
    DOWN = 2
    LEFT = 3
    RIGHT = 4

vis_pos: list[(int, int)] = []

def find_basin_size(x: int, y: int, prev_loc = PreviousLocation.NONE) -> int:
    size = 1
    if heightmap[y][x - 1] > heightmap[y][x] and heightmap[y][x - 1] != 9 and prev_loc != PreviousLocation.LEFT:
        if (x - 1, y) not in vis_pos:
            vis_pos.append((x - 1, y))
            size += find_basin_size(x - 1, y,PreviousLocation.RIGHT)
    if heightmap[y][x + 1] > heightmap[y][x] and heightmap[y][x + 1] != 9 and prev_loc != PreviousLocation.RIGHT:
        if (x + 1, y) not in vis_pos:
            vis_pos.append((x + 1, y))
            size += find_basin_size(x + 1, y,PreviousLocation.LEFT)
    if heightmap[y - 1][x] > heightmap[y][x] and heightmap[y - 1][x] != 9 and prev_loc != PreviousLocation.DOWN:
        if (x, y - 1) not in vis_pos:
            vis_pos.append((x, y - 1))
            size += find_basin_size(x, y - 1, PreviousLocation.UP)
    if heightmap[y + 1][x] > heightmap[y][x] and heightmap[y + 1][x] != 9 and prev_loc != PreviousLocation.UP:
        if (x, y + 1) not in vis_pos:
            vis_pos.append((x, y + 1))
            size += find_basin_size(x, y + 1, PreviousLocation.DOWN)
    return size

basin_sizes = [find_basin_size(*low_point) for low_point in low_points]

basin_sizes.sort()
result = 1
for size in basin_sizes[-3:]:
    result *= size

print(result)