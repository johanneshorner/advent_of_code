

file = open("input.in", "r")

input = file.read()

commands = [item.split(' ') for item in input.split('\n')]

forward = 0
depth = 0

for command in commands:
    match command[0]:
        case "forward":
            forward += int(command[1])
        case "down":
            depth += int(command[1])
        case "up":
            depth -= int(command[1])

print(forward * depth)