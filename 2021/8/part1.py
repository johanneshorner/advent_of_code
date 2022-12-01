with open("input.in") as file:
    lines = [lines.strip().split(" | ") for lines in file]
    lines = [output_digits.split() for inputs, output_digits in lines]

sum_of_1478 = 0

for output_digits in lines:
    for digit in output_digits:
        if len(digit) == 2 or len(digit) == 3 or len(digit) == 4 or len(digit) == 7:
            sum_of_1478 += 1

print(sum_of_1478)
