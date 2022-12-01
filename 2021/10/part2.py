with open("input.in") as file:
    lines = [[chunk_part for chunk_part in line.strip()] for line in file]

opening_lookup = {"(" : ")", "[" : "]", "{" : "}", "<" : ">"}
closing_lookup = {")" : "(", "]" : "[", "}" : "{", ">" : "<"}

complete_lookup = {")" : 1, "]" : 2, "}" : 3, ">" : 4}

def get_completion_score(line: list[str]) -> int:
    opening_stack = []
    for chunk_part in line:
        if chunk_part in opening_lookup:
            opening_stack.append(chunk_part)
        else:
            if len(opening_stack) != 0:
                if opening_lookup[opening_stack.pop()] == chunk_part:
                    continue
            return 0
    sub_sum = 0
    for _ in range(len(opening_stack)):
        sub_sum = sub_sum * 5 + complete_lookup[opening_lookup[opening_stack.pop()]]
    return sub_sum

sub_sums = [score for score in (get_completion_score(line) for line in lines) if score != 0]
sub_sums.sort()
print(sub_sums[int(len(sub_sums)/2)])


            