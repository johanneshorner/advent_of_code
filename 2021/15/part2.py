import math
import heapq

with open("input.in") as file:
    risk_level_grid = [[int(risk_level) for risk_level in line.strip()] for line in file]

def dikstra(grid, start):
    distances = {(x, y) : math.inf for y in range(len(grid)) for x in range(len(grid[0]))}

    heap = [(0, start)]

    while heap:
        distance, (x, y) = heapq.heappop(heap)
        
        #already found shorter path
        if distance > distances[(x, y)]:
            continue

        if (len(grid) - 1, len(grid[0]) - 1) == (x, y):
            return distance

        if x < len(grid[0]) - 1:
            neighbor = (x + 1, y)
            if distance + grid[y][x + 1] < distances[neighbor]:
                distances[neighbor] = distance + grid[y][x + 1]
                heapq.heappush(heap, (distances[neighbor], neighbor))
        if y < len(grid) - 1:
            neighbor = (x, y + 1)
            if distance + grid[y + 1][x] < distances[neighbor]:
                distances[neighbor] = distance + grid[y + 1][x]
                heapq.heappush(heap, (distances[neighbor], neighbor))
        if x > 0:
            neighbor = (x - 1, y)
            if distance + grid[y][x - 1] < distances[neighbor]:
                distances[neighbor] = distance + grid[y][x - 1]
                heapq.heappush(heap, (distances[neighbor], neighbor))
        if y > 0:
            neighbor = (x, y - 1)
            if distance + grid[y - 1][x] < distances[neighbor]:
                distances[neighbor] = distance + grid[y - 1][x]
                heapq.heappush(heap, (distances[neighbor], neighbor))

    return distances[(len(grid) - 1, len(grid[0]) - 1)]

risk_level_grid = [
    [x + j + i if x + j + i <= 9 else (x + j + i) - 9 for j in range(5) for x in y]
    for i in range(5) for y in risk_level_grid
]

print(dikstra(risk_level_grid, (0, 0)))