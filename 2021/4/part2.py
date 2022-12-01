def find_winning_board_and_winning_number(boards: list, draws: list[int]):
    indexes_of_done_boards = []

    for i in range(0, len(draws), 1):
        for j in range(0, len(boards), 1):
            for r in range(0, len(boards[j]), 1):
                for n in range(0, len(boards[j][r]), 1):
                    if draws[i] == boards[j][r][n]:
                        boards[j][r][n] = 100

            for r1 in range(0, len(boards[j]), 1):
                bingo = 0
                for n1 in range(0, len(boards[j][r1]), 1):
                    if boards[j][r1][n1] == 100:
                        bingo += 1
                        if bingo == 5:
                            if j not in indexes_of_done_boards:
                                indexes_of_done_boards.append(j)
                                if len(indexes_of_done_boards) == len(boards):
                                    return [boards[j], draws[i]]

            for r1 in range(0, len(boards[j]), 1):
                bingo = 0
                for n1 in range(0, len(boards[j][r1]), 1):
                    if boards[j][n1][r1] == 100: 
                        bingo += 1
                        if bingo == 5: 
                            if j not in indexes_of_done_boards:
                                indexes_of_done_boards.append(j)
                                if len(indexes_of_done_boards) == len(boards):
                                    return [boards[j], draws[i]]
            

    return []



file = open("input.in", "r")

input = file.read().replace("  ", " ").split("\n\n")

draws = [int(draw) for draw in input[0].split(',')]

boards = []

for i in range(1, len(input), 1):
    str_board = [str(row).lstrip() for row in input[i].split('\n')]
    board = []
    for row in str_board:
        board.append([int(number) for number in row.split(' ')])
    boards.append(board)

[winning_board, winning_number] = find_winning_board_and_winning_number(boards, draws)

sum = 0

for row in winning_board:
    for number in row:
        if number != 100:
            sum += number

print(sum * winning_number)
            
