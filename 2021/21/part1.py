import copy

class Player:
    def __init__(self, pos) -> None:
        self.pos = pos
        self.score = 0

class Game:
    def throw_dice(self):
        sum = 0
        for _ in range(3):
            self.dice_throw_count += 1
            sum += self.dice_value
            self.dice_value += 1
            if self.dice_value == 101:
                self.dice_value = 1

        return sum

    def do_turn(self, player: Player):
        player.pos += self.throw_dice()
        if player.pos > 10:
            if player.pos % 10 == 0:
                player.pos = 10
            else:
                player.pos = player.pos % 10
        player.score += player.pos

    def run(self):
        while True:
            self.do_turn(self.p1)
            if self.p1.score >= 1000:
                return self.p2.score, self.dice_throw_count
            self.do_turn(self.p2)
            if self.p2.score >= 1000:
                return self.p1.score, self.dice_throw_count

    def __init__(self, p1_pos, p2_pos) -> None:
        self.p1 = Player(p1_pos)
        self.p2 = Player(p2_pos)
        self.dice_value = 1
        self.dice_throw_count = 0

def get_input():
    with open('input.in', 'r') as file:
        start_positions = [int(line.strip().split(': ')[1]) for line in file]
        
        return start_positions

game = Game(*get_input())

p1_score, p2_score = game.run()

print(p1_score * p2_score)
