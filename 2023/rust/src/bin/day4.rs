type Input = Vec<Game>;
type Output = u32;

#[derive(Debug, Clone)]
struct Game {
    winning_numbers: Vec<u32>,
    guesses: Vec<u32>,
}

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| {
            let (_, cards) = line.split_once(": ").unwrap();
            let [winning_numbers, guesses] = &cards
                .split(" | ")
                .map(|card| {
                    card.split_whitespace()
                        .map(|number| number.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>()[..]
            else {
                unreachable!("bad input");
            };

            Game {
                winning_numbers: winning_numbers.clone(),
                guesses: guesses.clone(),
            }
        })
        .collect::<Vec<Game>>()
}

fn part1(input: Input) -> Output {
    let mut sum = 0;

    for game in input {
        let matches = game
            .guesses
            .iter()
            .filter(|guess| {
                game.winning_numbers
                    .iter()
                    .find(|number| number == guess)
                    .is_some()
            })
            .count();
        if matches > 0 {
            sum = sum + 2u32.pow(matches as u32 - 1);
        }
    }

    sum
}

fn part2(input: Input) -> Output {
    let mut cards = vec![1; input.len()];

    for (i, game) in input.iter().enumerate() {
        let matches = game
            .guesses
            .iter()
            .filter(|guess| {
                game.winning_numbers
                    .iter()
                    .find(|number| number == guess)
                    .is_some()
            })
            .count();
        for j in 1..=matches {
            cards[i + j] = cards[i + j] + cards[i];
        }
    }

    cards.iter().sum()
}

fn main() {
    let input = parse_input(include_str!("../../../input/day4.in"));
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    const PART1_SOLUTION: u32 = 13;
    const PART2_SOLUTION: u32 = 30;

    #[test]
    fn test_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(input), PART1_SOLUTION);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(input), PART2_SOLUTION);
    }
}
