use regex::Regex;

fn main() {
    let input = include_str!("../../../input/day3.txt");
    eprintln!("{}", part1(&input));
    eprintln!("{}", part2(&input));
}

type Input = str;

fn part1(input: &Input) -> usize {
    let re = Regex::new(r#"mul\((\d*),(\d*)\)"#).unwrap();
    input
        .trim()
        .split("\n")
        .map(|line| {
            re.captures_iter(line)
                .map(|caps| {
                    let (_, [l, r]) = caps.extract();
                    l.parse::<usize>().unwrap() * r.parse::<usize>().unwrap()
                })
                .sum::<usize>()
        })
        .sum()
}

fn part2(input: &Input) -> usize {
    let re = Regex::new(r#"mul\((\d*),(\d*)\)|don't\(\)|do\(\)"#).unwrap();
    input
        .trim()
        .split("\n")
        .map(|line| {
            let mut enabled = true;
            let mut sum = 0;
            for caps in re.captures_iter(line) {
                match &caps[0] {
                    "do()" => enabled = true,
                    "don't()" => enabled = false,
                    _ => {
                        if enabled {
                            sum += caps[1].parse::<usize>().unwrap()
                                * caps[2].parse::<usize>().unwrap()
                        }
                    }
                }
            }
            sum
        })
        .sum()
}

#[cfg(test)]
mod tests {
    static INPUT: &'static str =
        r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 161);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 48);
    }
}
