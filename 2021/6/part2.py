input = open("input.in", "r").read()

days = {day : 0 for day in range(9)}

for fish in input.split(','):
    days[int(fish)] += 1

for _ in range(256):
    day_0 = days[0]
    for i in range(1, 9, 1):
        days[i - 1] = days[i]
    days[6] += day_0
    days[8] = day_0

sum = 0

for day in days:
    sum += days[day]

print(sum)