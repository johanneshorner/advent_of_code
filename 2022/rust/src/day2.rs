pub fn run(input_lines: &[&str]) {
    let input1 = parse_input1(input_lines);
    let input2 = parse_input2(input_lines);

    println!("Part1: {}", part(&input1));
    println!("Part2: {}", part(&input2));
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone, Copy, Debug)]
struct Round {
    you: Shape,
    opponent: Shape,
}

fn parse_input1(input_lines: &[&str]) -> Vec<Round> {
    input_lines
        .iter()
        .map(|line| {
            let shapes: Vec<Shape> = line
                .split(" ")
                .map(|shape_char| match shape_char {
                    "A" | "X" => Shape::Rock,
                    "B" | "Y" => Shape::Paper,
                    "C" | "Z" => Shape::Scissors,
                    _ => unreachable!(),
                })
                .collect();

            Round {
                you: shapes[1],
                opponent: shapes[0],
            }
        })
        .collect()
}

fn parse_input2(input_lines: &[&str]) -> Vec<Round> {
    input_lines
        .iter()
        .map(|line| {
            let shapes: Vec<&str> = line.split(" ").collect();

            let shapes = (
                match shapes[0] {
                    "A" => Shape::Rock,
                    "B" => Shape::Paper,
                    "C" => Shape::Scissors,
                    _ => unreachable!(),
                },
                shapes[1],
            );

            let get_winning_shape = |shape: Shape| {
                match shape {
                    Shape::Rock => Shape::Paper,
                    Shape::Paper => Shape::Scissors,
                    Shape::Scissors => Shape::Rock,
                }
            };

            Round {
                you: match shapes {
                    (opponent, "X") => get_winning_shape(get_winning_shape(opponent)),
                    (opponent, "Y") => opponent,
                    (opponent, "Z") => get_winning_shape(opponent),
                    _ => unreachable!(),
                },
                opponent: shapes.0,
            }
        })
        .collect()
}

fn simulate_round(round: &Round) -> usize {
    let result: usize = match round {
        Round { you, opponent } if you == opponent => 3,
        Round {
            you: Shape::Rock,
            opponent: Shape::Scissors,
        }
        | Round {
            you: Shape::Paper,
            opponent: Shape::Rock,
        }
        | Round {
            you: Shape::Scissors,
            opponent: Shape::Paper,
        } => 6,
        _ => 0,
    };

    result + round.you as usize
}

fn part(rounds: &[Round]) -> usize {
    rounds.iter().map(|round| simulate_round(round)).sum()
}

#[cfg(test)]
mod tests {
    use super::{parse_input1, parse_input2};

    #[test]
    fn part1() {
        let input_lines = vec!["A Y", "B X", "C Z"];

        let input = parse_input1(&input_lines);
        let result = super::part(&input);

        assert_eq!(15, result);
    }

    #[test]
    fn part2() {
        let input_lines = vec!["A Y", "B X", "C Z"];

        let input = parse_input2(&input_lines);
        let result = super::part(&input);

        assert_eq!(12, result);
    }
}
