use itertools::*;

fn main() {
    let input = parse_input(include_str!("../../../input/day1.txt"));
    eprintln!("{}", part1(&input));
    eprintln!("{}", part2(&input));
}

type Input = (Vec<u32>, Vec<u32>);

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let (l, r) = line.split_once("   ").unwrap();
            (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap())
        })
        .unzip()
}

fn part1((ref l, ref r): &Input) -> u32 {
    l.iter()
        .sorted()
        .zip(r.iter().sorted())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

fn part2((ref l, ref r): &Input) -> usize {
    l.iter()
        .map(|n| r.iter().filter(|other| n == *other).count() * *n as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    static INPUT: &'static str = r#"3   4
4   3
2   5
1   3
3   9
3   3
        "#;

    #[test]
    fn part1() {
        assert_eq!(super::part1(&parse_input(INPUT)), 11)
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&parse_input(INPUT)), 31)
    }
}
