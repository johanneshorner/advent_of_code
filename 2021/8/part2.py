with open("input.in") as file:
    lines = [lines.strip().split(" | ") for lines in file]
    lines = [(inputs.split(), output_digits.split()) for inputs, output_digits in lines]

def solve(inputs, output_digits):
    digits = [None] * 10
    inputs = [set(i) for i in inputs]

    digits[1] = next(i for i in inputs if len(i) == 2)
    digits[4] = next(i for i in inputs if len(i) == 4)
    digits[7] = next(i for i in inputs if len(i) == 3)
    digits[8] = next(i for i in inputs if len(i) == 7)

    digits[3] = next(i for i in inputs if len(i) == 5 and i > digits[1])
    digits[9] = next(i for i in inputs if len(i) == 6 and i > digits[3])
    digits[0] = next(i for i in inputs if len(i) == 6 and i not in digits and i > digits[1])
    digits[6] = next(i for i in inputs if len(i) == 6 and i not in digits)
    digits[5] = next(i for i in inputs if len(i) == 5 and i < digits[6])
    digits[2] = next(i for i in inputs if len(i) == 5 and i not in digits)

    output_digits = [set(od) for od in output_digits]
    output = 0

    for od in output_digits:
        output = output * 10 + next(idx for idx,digit in enumerate(digits) if od == digit)

    return output

print(sum(solve(inputs, output_digits) for inputs, output_digits in lines))