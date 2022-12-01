with open("input.in") as file:
    lines = [[chunk_part for chunk_part in line.strip()] for line in file]

opening_lookup = {"(" : ")", "[" : "]", "{" : "}", "<" : ">"}
closing_lookup = {")" : "(", "]" : "[", "}" : "{", ">" : "<"}

error_point_lookup = {")" : 3, "]" : 57, "}" : 1197, ">" : 25137}

error_sum = 0

for line in lines:
    opening_stack = []
    for chunk_part in line:
        if chunk_part in opening_lookup:
            opening_stack.append(chunk_part)
        else:
            if len(opening_stack) != 0:
                if opening_lookup[opening_stack.pop()] == chunk_part:
                    continue
            error_sum += error_point_lookup[chunk_part]
            break

print(error_sum)

            