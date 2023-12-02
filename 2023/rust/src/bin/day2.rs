use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    rounds: Vec<HashMap<String, u32>>,
}

type Input = Vec<Game>;
type Output = u32;

fn parse_input(input: &str) -> Input {
    let input = input
        .trim()
        .split('\n')
        .map(|line| {
            let (game, rest) = line.split_once(':').unwrap();
            let id = game.split_once(' ').unwrap().1.parse::<u32>().unwrap();

            let rounds = rest
                .split(';')
                .map(|round| {
                    HashMap::from_iter(round.trim().split(',').map(|color| {
                        let (count, color) = color.trim().split_once(' ').unwrap();
                        (color.to_string(), count.parse::<u32>().unwrap())
                    }))
                })
                .collect::<Vec<_>>();
            Game { id, rounds }
        })
        .collect::<Vec<_>>();
    println!("{input:?}");
    input
}

fn part1(input: Input) -> Output {
    let (red, green, blue) = (12, 13, 14);

    let mut sum = 0;

    for game in input {
        let mut possible = true;
        for round in game.rounds {
            if (round.get("red").is_some() && round["red"] > red)
                || (round.get("green").is_some() && round["green"] > green)
                || (round.get("blue").is_some() && round["blue"] > blue)
            {
                possible = false;
                break;
            }
        }
        if possible {
            sum += game.id;
        }
    }

    sum
}

fn part2(input: Input) -> Output {
    let mut sum = 0;

    for game in input {
        let mut red_max = 0;
        let mut green_max = 0;
        let mut blue_max = 0;
        for round in game.rounds {
            if round.get("red").is_some() && round["red"] > red_max {
                red_max = round["red"];
            }
            if round.get("green").is_some() && round["green"] > green_max {
                green_max = round["green"];
            }
            if round.get("blue").is_some() && round["blue"] > blue_max {
                blue_max = round["blue"];
            }
        }
        sum += red_max * green_max * blue_max;
    }

    sum
}

fn main() {
    let input = parse_input(include_str!("../../../input/day2.in"));
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_INPUT: &str = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";
    const PART2_INPUT: &str = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";
    const PART1_SOLUTION: u32 = 8;
    const PART2_SOLUTION: u32 = 2286;

    #[test]
    fn test_part1() {
        let input = parse_input(PART1_INPUT);
        assert_eq!(part1(input), PART1_SOLUTION);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(PART2_INPUT);
        assert_eq!(part2(input), PART2_SOLUTION);
    }
}
