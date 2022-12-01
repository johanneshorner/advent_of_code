import copy

def get_input():
    with open('input.in', 'r') as file:
        algo_lines, input_image = file.read().split('\n\n')
        
        return (''.join([line.strip() for line in algo_lines]), [[c for c in r.strip()] for r in input_image.split()])

def add_border(image: list[list[str]], value, size):
    new_image = copy.deepcopy(image)

    for _ in range(size):
        for row in new_image:
            row.insert(0, value)
            row.append(value)

        new_image.insert(0, [value] * len(new_image[0]))
        new_image.append([value] * len(new_image[0]))

    return new_image

def remove_border(image: list[list[str]], size):
    new_image = copy.deepcopy(image)

    for i in range(len(new_image)):
        new_image[i] = new_image[i][size:-size]

    new_image = new_image[size:-size]

    return new_image

def enhance(input_image: list[list[str]], start_index):
    output_image = copy.deepcopy(input_image)

    for r in range(start_index, len(input_image) - start_index):
        for c in range(start_index, len(input_image[0]) - start_index):
            pixels = \
                "".join(input_image[r - 1][c - 1:c + 2]) \
                + "".join(input_image[r][c - 1:c + 2])\
                + "".join(input_image[r + 1][c - 1:c + 2])

            binary_number = "".join(["0" if char == "." else "1" for char in pixels])

            output_image[r][c] = algo_lookup[int(binary_number, 2)]

    return output_image

algo_lookup, image = get_input()

for i in range(2):
    if i % 2 == 0:
        image = enhance(add_border(image, '.', 5), 4)
        image = remove_border(image, 4)
    else:
        image = enhance(add_border(image, '#', 5), 4)
        image = remove_border(image, 4)

print([pixel for row in image for pixel in row].count('#'))