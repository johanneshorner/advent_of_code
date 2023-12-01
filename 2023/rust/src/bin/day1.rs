type Input = Vec<String>;
type Output = u32;

fn parse_input(input: &str) -> Input {
    let input = input
        .trim()
        .split('\n')
        .map(|line| line.to_string())
        .collect::<Vec<_>>();
    input
}

fn part1(input: Input) -> Output {
    let mut sum = 0;
    for line in input {
        let mut digits = Vec::new();
        for c in line.chars() {
            if c.is_digit(10) {
                digits.push(c);
            }
        }
        let number = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
        sum = sum + number.parse::<u32>().unwrap();
    }

    sum
}

fn part2(input: Input) -> Output {
    let mut sum = 0;
    for line in input {
        let mut digits = Vec::new();
        for (i, c) in line.char_indices() {
            if c.is_digit(10) {
                digits.push(c);
            } else {
                let numbers = [
                    ("one", '1'),
                    ("two", '2'),
                    ("three", '3'),
                    ("four", '4'),
                    ("five", '5'),
                    ("six", '6'),
                    ("seven", '7'),
                    ("eight", '8'),
                    ("nine", '9'),
                ];
                for number in numbers {
                    if let Some(pos) = &line[i..].find(&number.0) {
                        if *pos == 0 {
                            digits.push(number.1);
                        }
                    }
                }
            }
        }
        let number = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
        sum = sum + number.parse::<u32>().unwrap();
    }

    sum
}

fn main() {
    let input = parse_input(include_str!("../../../input/day1.in"));
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    const PART2_INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    const PART1_SOLUTION: u32 = 142;
    const PART2_SOLUTION: u32 = 281;

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
