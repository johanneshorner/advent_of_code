

file = open("input.in", "r")

input = file.read()

depths = [int(item) for item in input.split('\n')]

depthIncreases = 0

for i in range(3, len(depths), 1):
    if((depths[i - 2] + depths[i - 1] + depths[i - 0]) > (depths[i - 3] + depths[i - 2] + depths[i - 1])):
        depthIncreases+=1

print(depthIncreases)