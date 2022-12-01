def get_points_and_fold_coords(input_str: str):
    point_lines = input_str.split("\n\n")[0].split("\n")
    fold_lines = input_str.split("\n\n")[1].split("\n")

    return (
        [(int(x), int(y)) for x,y in (point.split(",") for point in point_lines)],
        [(name, int(xy)) for name,xy in (fold_line[fold_line.find("=") - 1:].split("=") for fold_line in fold_lines)]
    )

with open("input.in") as file:
    points, folds = get_points_and_fold_coords(file.read())

def fold(points, axis, n):
    if axis == "x":
        return {(n-(x-n), y) if x > n else (x, y) for x, y in points}
    return {(x, n-(y-n)) if y > n else (x, y) for x, y in points}

for axis, n in folds:
    points = fold(points, axis, n)

side_len = max([n for point in points for n in point]) + 1

paper = [[" "] * side_len for y in range(side_len)]

for y in range(side_len):
    for x in range(side_len):
        if (x, y) in points:
            paper[y][x] = "#"

with open("output.txt", "w") as file:
    for row in paper:
        file.write("".join(row) + "\n")