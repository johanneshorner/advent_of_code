input = open("input.in", "r").read()

fishes = [int(fish) for fish in input.split(',')]

for d in range(256):
    print(d)
    for i in range(len(fishes)):
        if fishes[i] == 0:
            fishes[i] = 6
            fishes.append(8)
        else:
            fishes[i] -= 1

print(len(fishes))