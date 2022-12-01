with open("input.in") as file:
    input = file.read().strip()
    values = [values[values.index("=")+1:].split("..") for values in input[input.index("x"):].split(", ")]
    x_bounds = [int(value) for value in values[0]]
    y_bounds = [int(value) for value in values[1]]


def apply_step(x, y, x_velocity, y_velocity):
    x += x_velocity
    y += y_velocity
    if x_velocity > 0:
        x_velocity -= 1
    y_velocity -= 1
    return x, y, x_velocity, y_velocity

def fire_probe(x, y, x_velocity, y_velocity, x_bounds, y_bounds):
    x_min, x_max = x_bounds
    y_max, y_min = y_bounds

    highest_y = 0

    while x <= x_max and y >= y_max:
        x, y, x_velocity, y_velocity = apply_step(x, y, x_velocity, y_velocity)
        if y > highest_y:
            highest_y = y
        if (x >= x_min and x <= x_max and y <= y_min and y >= y_max):
            return (True, highest_y)

    return (False, highest_y)

ys = []
for x in range(10000):
    for y in range(-200, 1000):
        hasHit, highest_y = fire_probe(0, 0, x, y, x_bounds, y_bounds)

        if hasHit:
            ys.append(highest_y)

print(max(ys))