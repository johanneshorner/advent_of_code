type Input = Vec<Race>;
type Input2 = Race;
type Output = u64;

#[derive(Debug, Clone)]
struct Race {
    duration: u64,
    distance: u64,
}

fn parse_input_1(input: &str) -> Input {
    let input = input
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut races = Vec::new();

    for i in 0..input[0].len() {
        races.push(Race {
            duration: input[0][i],
            distance: input[1][i],
        });
    }

    races
}

fn part1(input: Input) -> Output {
    let mut win_counts = Vec::new();

    for race in input {
        let mut win_count = 0;
        for hold_duration in 1..race.duration {
            let distance = (race.duration - hold_duration) * hold_duration;
            if distance > race.distance {
                win_count = win_count + 1;
            }
        }
        if win_count > 0 {
            win_counts.push(win_count);
        }
    }

    win_counts.iter().product()
}

fn parse_input_2(input: &str) -> Input2 {
    let input = input
        .replace(" ", "")
        .lines()
        .map(|line| line.split_once(':').unwrap().1.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    Race {
        duration: input[0],
        distance: input[1],
    }
}

fn part2(input: Input2) -> Output {
    let mut win_count = 0;
    for hold_duration in 1..input.duration {
        let distance = (input.duration - hold_duration) * hold_duration;
        if distance > input.distance {
            win_count = win_count + 1;
        }
    }

    win_count
}

fn main() {
    let input = parse_input_1(include_str!("../../../input/day6.in"));
    println!("Part 1: {}", part1(input.clone()));
    let input = parse_input_2(include_str!("../../../input/day6.in"));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";
    const INPUT_2: &str = "Time:      7  15   30
Distance:  9  40  200";
    const PART1_SOLUTION: Output = 288;
    const PART2_SOLUTION: Output = 71503;

    #[test]
    fn test_part1() {
        let input = parse_input_1(INPUT);
        assert_eq!(part1(input), PART1_SOLUTION);
    }

    #[test]
    fn test_part2() {
        let input = parse_input_2(INPUT_2);
        assert_eq!(part2(input), PART2_SOLUTION);
    }
}
