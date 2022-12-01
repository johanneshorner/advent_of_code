def draw_line(field: list[int], side_len, x1, y1, x2, y2):
    if x1 == x2:
        for i in range(min(y1, y2) * side_len + x1, max(y1, y2) * side_len + x1 + 1, side_len):
            field[i] += 1
    elif y1 == y2:
        for i in range(min(x1, x2) + side_len * y1, max(x1, x2) + side_len * y1 + 1, 1):
            field[i] += 1
    else:
        p1 = (x1, y1)
        p2 = (x2, y2)
        start_p = p1 if y1 < y2 else p2
        stop_p = p1 if y1 > y2 else p2
        for i in range(start_p[0] + side_len * start_p[1], stop_p[0] + side_len * stop_p[1] + 1, side_len - 1 if start_p[0] > stop_p[0] else side_len + 1):
            field[i] += 1 

file = open("input.in", "r")

lines = file.read().split("\n")

vent_lines = [[tuple(map(lambda n: int(n), point.split(','))) for point in vent_line.split(" -> ")] for vent_line in lines]

n_max = max([n for vent_line in vent_lines for point in vent_line for n in point]) + 1

field = [0 for r in range(n_max * n_max)]

for vent_line in vent_lines:
    draw_line(field, n_max, *vent_line[0], *vent_line[1])

field1 = [[field[n1] for n1 in range(n, n + n_max, 1)] for n in range(0, len(field), n_max)]

print(sum(n >= 2 for n in field))