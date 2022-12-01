import itertools
import functools

def get_input():
    with open('input.in', 'r') as file:
        start_positions = [int(line.strip().split(': ')[1]) - 1 for line in file]
        
        return start_positions

role_table = list(itertools.product([1, 2, 3], repeat=3))

@functools.cache
def play(current_player_pos, next_player_pos, current_player_score, next_player_score):
    if current_player_score >= 21:
        return (1, 0)
    if next_player_score >= 21:
        return (0, 1)

    sum_p1_wins = sum_p2_wins = 0

    for r1, r2, r3 in role_table:
        new_pos = (current_player_pos + r1 + r2 + r3) % 10
        new_score = current_player_score + new_pos + 1

        p2_wins, p1_wins = play(next_player_pos, new_pos, next_player_score, new_score)

        sum_p1_wins += p1_wins
        sum_p2_wins += p2_wins

    return (sum_p1_wins, sum_p2_wins)


p1, p2 = get_input()

print(max(play(p1, p2, 0, 0)))
