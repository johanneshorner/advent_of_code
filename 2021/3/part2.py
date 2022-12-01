

file = open("input.in", "r")

input = file.read()

numbers = input.split('\n')

numbers2 = numbers

for i in range(0, len(numbers[0]), 1):
    numbers_0 = []
    numbers_1 = []

    for number in numbers:
        if number[i] == '0':
            numbers_0.append(number)
        else:
            numbers_1.append(number)

    if len(numbers_1) > len(numbers_0):
        numbers = numbers_1
    elif len(numbers_1) == len(numbers_0):
        numbers = numbers_1
    else:
        numbers = numbers_0

    if(len(numbers) == 1): break

ox_gen_rating = numbers[0]

numbers = numbers2

for i in range(0, len(numbers[0]), 1):
    numbers_0 = []
    numbers_1 = []

    for number in numbers:
        if number[i] == '0':
            numbers_0.append(number)
        else:
            numbers_1.append(number)

    if len(numbers_0) < len(numbers_1):
        numbers = numbers_0
    elif len(numbers_0) == len(numbers_1):
        numbers = numbers_0
    else:
        numbers = numbers_1

    if(len(numbers) == 1): break

print(int(ox_gen_rating, 2) * int(numbers[0], 2))