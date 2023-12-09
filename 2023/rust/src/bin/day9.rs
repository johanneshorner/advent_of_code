type Input = Vec<Vec<i64>>;
type Output = i64;

fn parse_input(input: &str) -> Input {
    let input = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    input
}

fn part1(input: Input) -> Output {
    let mut sum = 0;

    for history in &input {
        let mut sequences = Vec::new();

        let mut previous_sequence = history;
        loop {
            let new_sequence = (0..previous_sequence.len() - 1)
                .map(|i| previous_sequence[i + 1] - previous_sequence[i])
                .collect::<Vec<_>>();
            sequences.push(new_sequence);
            previous_sequence = sequences.last().unwrap();
            if previous_sequence.iter().all(|value| *value == 0) {
                break;
            }
        }

        let new_value = sequences
            .iter()
            .map(|sequence| sequence.last().unwrap())
            .sum::<i64>()
            + history.last().unwrap();
        sum += new_value;
    }

    sum
}

fn part2(input: Input) -> Output {
    let mut sum = 0;

    let input = input
        .iter()
        .map(|history| history.iter().cloned().rev().collect::<Vec<i64>>())
        .collect::<Vec<_>>();

    for history in &input {
        let mut sequences = Vec::new();

        let mut previous_sequence = history;
        loop {
            let new_sequence = (0..previous_sequence.len() - 1)
                .map(|i| previous_sequence[i + 1] - previous_sequence[i])
                .collect::<Vec<_>>();
            sequences.push(new_sequence);
            previous_sequence = sequences.last().unwrap();
            if previous_sequence.iter().all(|value| *value == 0) {
                break;
            }
        }

        let new_value = sequences
            .iter()
            .map(|sequence| sequence.last().unwrap())
            .sum::<i64>()
            + history.last().unwrap();
        sum += new_value;
    }

    sum
}

fn main() {
    let input = parse_input(include_str!("../../../input/day9.in"));
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    const PART1_SOLUTION: Output = 114;
    const PART2_SOLUTION: Output = 2;

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
