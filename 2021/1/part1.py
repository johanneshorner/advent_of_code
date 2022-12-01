

file = open("input.in", "r")

input = file.read()

depths = [int(item) for item in input.split('\n')]

depthIncreases = 0

for i in range(1, len(depths), 1):
    if(depths[i] > depths[i - 1]):
        depthIncreases+=1

print(depthIncreases)