pub fn run(input_lines: &[&str]) {
    let input = parse_input(input_lines[0]);

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn parse_input(input_line: &str) -> Vec<char> {
    input_line.chars().collect()
}

use std::collections::HashSet;

fn part1(chars: &[char]) -> usize {
    for i in 0..chars.len() {
        let marker_size = 4;
        let sequence: HashSet<char> = HashSet::from_iter(chars[i..i+marker_size].iter().cloned());
        if sequence.len() == marker_size {
            return i + marker_size;
        }
    }

    unreachable!();
}

fn part2(chars: &[char]) -> usize {
    for i in 0..chars.len() {
        let marker_size = 14;
        let sequence: HashSet<char> = HashSet::from_iter(chars[i..i+marker_size].iter().cloned());
        if sequence.len() == marker_size {
            return i + marker_size;
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    const INPUT_LINE: &'static str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn part1() {
        let input = parse_input(&INPUT_LINE);
        let result = super::part1(&input);

        assert_eq!(7, result);
    }

    #[test]
    fn part2() {
        let input = parse_input(&INPUT_LINE);
        let result = super::part2(&input);

        assert_eq!(19, result);
    }
}
