with open("input.in") as file:
    lines = [[int(c) for c in line.strip()] for line in file]

def solve(area: list[list[int]]) -> int:
    def process_flashes(area: list[list[int]]) -> tuple[list[list[int]], bool]:
        flash_count = 0
        flash_pos = set()
        while True:
            flash = False
            for r in range(len(area)):
                for c in range(len(area[0])):
                    if (r, c) not in flash_pos and area[r][c] > 9:
                        flash = True
                        flash_pos.add((r, c))

                        if c > 0:
                            area[r][c - 1] += 1
                        if c > 0 and r > 0:
                            area[r - 1][c - 1] += 1
                        if r > 0:
                            area[r - 1][c] += 1
                        if r > 0 and c < len(area[0]) - 1:
                            area[r - 1][c + 1] += 1
                        if c < len(area[0]) - 1:
                            area[r][c + 1] += 1
                        if r < len(area) - 1 and c < len(area[0]) - 1:
                            area[r + 1][c + 1] += 1
                        if r < len(area) - 1:
                            area[r + 1][c] += 1
                        if r < len(area) - 1 and c > 0:
                            area[r + 1][c - 1] += 1
            if not flash:
                break
        if len(flash_pos) == len(area) * len(area[0]):
            return (area, True)
        for r in range(len(area)):
            for c in range(len(area[0])):
                if (r, c) in flash_pos:
                    area[r][c] = 0
        return (area, False)

    for i in range(10000):
        area = [[n + 1 for n in rows] for rows in area]
        area, flash_sync = process_flashes(area)
        if flash_sync: 
            break
    
    return i

print(solve(lines) + 1)