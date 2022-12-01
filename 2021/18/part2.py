import functools

def tokenize(string: str):
    tokens = []
    i = 0
    while i < len(string):
        if string[i].isnumeric():
            if string[i + 1].isnumeric():
                tokens.append(int(string[i:i + 2]))
                i += 1
            else:
                tokens.append(int(string[i]))
        else:
            tokens.append(string[i])
        i += 1
    return tokens

def tokens_to_str(tokens: list):
    return "".join([str(token) for token in tokens])

with open("input.in") as file:
    sf_numbers = [tokenize(line.strip()) for line in file]

def explode_numbers(number: list, idx, left, right):
    for i in range(idx - 1, 0, -1):
        if isinstance(number[i], int):
            number = number[:i] + [number[i] + left] + number[i + 1:]
            break
    for i in range(idx + 2, len(number)):
        if isinstance(number[i], int):
            number = number[:i] + [number[i] + right] + number[i + 1:]
            break
    return number

def reduce(number: list):
    #explode
    while True:
        number_old = number
        nest_count = -1 #first bracket doesn't count
        for i in range(len(number) - 2):
            if number[i] == "[": nest_count += 1
            if number[i] == "]": nest_count -= 1
            if nest_count >= 4:
                if isinstance(number[i], int) and isinstance(number[i + 2], int):
                    left = number[i]
                    right = number[i + 2]
                    number = number[:i - 1] + [0] + number[i + 4:]
                    number = explode_numbers(number, i - 1, left, right)
                    break

        if number_old == number:
            #split
            for i in range(len(number) - 1):
                if isinstance(number[i], int):
                    if number[i] > 9:
                        left = int(number[i] / 2) #rounded down
                        right = number[i] - left #rounded up
                        number = number[:i] + ["["] + [left] + [","] + [right] + ["]"] + number[i + 1:]
                        break

        if number_old == number:
            break

    return number

def add(left, right) -> list:
    new_number = ["["] + left + [","] + right + ["]"]
    
    new_number = reduce(new_number)

    return new_number

result = functools.reduce(lambda a, b: add(a, b), sf_numbers)

def calc_magnitude(sf_number):
    return eval(sf_number.replace("[", "(3*").replace(",", " + 2*").replace("]", ")"))

sums = []

for number_left in sf_numbers:
    for number_right in sf_numbers:
        sums.append(calc_magnitude(tokens_to_str(add(number_left, number_right))))
        sums.append(calc_magnitude(tokens_to_str(add(number_right, number_left))))

print(max(sums))