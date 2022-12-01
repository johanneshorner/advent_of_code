

file = open("input.in", "r")

input = file.read()

numbers = input.split('\n')

gamma_rate = ""
epsilon_rate = ""

for i in range(0, len(numbers[0]), 1):
    count = 0

    for number in numbers:
        if ( number[i] == '0'):
            count -= 1
        else:
            count += 1

    if (count < 0):
        gamma_rate += '0'
        epsilon_rate += '1'
    else:
        gamma_rate += '1'
        epsilon_rate += '0'

print(int(gamma_rate, 2) * int(epsilon_rate, 2))