type Input = Vec<i32>;
type Output = i32;

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_input(input: &str) -> Input {
    let input = input
        .trim()
        .split('\n')
        .map(|line| {
            let [x, y] = line.split_whitespace().collect::<Vec<_>>()[..] else {
                panic!("Weird input");
            };
            Point {
                x: i32::from_str_radix(x, 10).unwrap(),
                y: i32::from_str_radix(y, 10).unwrap(),
            }
        })
        .collect::<Vec<_>>();
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
    let input = parse_input(include_str!("../../../input/day1.in"));
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "";
    const PART1_SOLUTION: i32 = 0;
    const PART2_SOLUTION: i32 = 1;

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE_INPUT);
        assert_eq!(part1(input), PART1_SOLUTION);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(EXAMPLE_INPUT);
        assert_eq!(part2(input), PART2_SOLUTION);
    }
}
