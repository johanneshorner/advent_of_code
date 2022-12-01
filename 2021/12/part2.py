with open("input.in") as file:
    lines = [tuple(line.strip().split("-")) for line in file]

def solve(cave_paths: list[tuple[str, str]]) -> int:
    caves_lookup = {cave : [] for cave in set([cave for paths in lines for cave in paths])}

    for path in cave_paths:
        for cave in caves_lookup:
            if path[0] == cave:
                caves_lookup[cave].append(path[1])
            elif path[1] == cave:
                caves_lookup[cave].append(path[0])

    found_paths = []

    def find_paths(current_cave: str, current_path: list[str]):
        def has_visited_lower_case_twice(path):
            lower_case_caves = set([cave for cave in path if cave.islower()])
            for cave in lower_case_caves:
                if path.count(cave) > 1:
                    return True
            return False

        new_path = current_path.copy()
        new_path.append(current_cave)
        if current_cave == "end":
            found_paths.append(new_path)
        else:
            for cave in caves_lookup[current_cave]:
                if cave != "start" and ((str(cave).islower() and (cave not in new_path or not has_visited_lower_case_twice(new_path))) or not str(cave).islower()):
                    find_paths(cave, new_path)

    find_paths("start", [])

    return len(found_paths)

print(solve(lines))