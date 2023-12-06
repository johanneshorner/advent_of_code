type Input = Vec<i32>;
type Output = i32;

fn parse_input(input: &str) -> Input {
    let input = input.lines().collect::<Vec<_>>();
    println!("{input:?}");
    vec![0]
}

fn part1(input: Input) -> Output {
    0
}

fn part2(input: Input) -> Output {
    1
}

fn main() {
    let input = parse_input(include_str!("../../../input/day.in"));
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";
    const PART1_SOLUTION: Output = 0;
    const PART2_SOLUTION: Output = 1;

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
