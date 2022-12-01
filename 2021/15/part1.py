import math

with open("input.in") as file:
    risk_level_graph = [[int(risk_level) for risk_level in line.strip()] for line in file]

def dikstra(graph):
    unvisited = set([(x, y) for y in range(len(graph)) for x in range(len(graph[0]))])
    distances = {node : math.inf for node in unvisited}

    distances[(0, 0)] = 0
    
    while (len(graph) - 1, len(graph[0]) - 1) in unvisited:
        temp_distances = {node : distances[node] for node in unvisited}
        x, y = min(temp_distances, key=temp_distances.get)
        
        if x < len(graph[0]) - 1:
            distances[(x + 1, y)] = min(distances[(x + 1, y)], distances[(x, y)] + graph[y][x + 1])
        if y < len(graph) - 1:
            distances[(x, y + 1)] = min(distances[(x, y + 1)], distances[(x, y)] + graph[y + 1][x])

        unvisited.remove((x, y))

    return distances[(len(graph) - 1, len(graph[0]) - 1)]

risk_level_graph[0][0] = 0
print(dikstra(risk_level_graph))