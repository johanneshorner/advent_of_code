use core::str;

fn main() {
    let input = parse_input(include_str!("../../../input/day7.txt"));
    eprintln!("{}", part1(&input));
    eprintln!("{}", part2(&input));
}

type Input = Vec<(usize, Vec<usize>)>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| {
            let (res, nums) = line.split_once(": ").unwrap();
            (
                res.parse::<usize>().unwrap(),
                nums.split(' ')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            )
        })
        .collect()
}

fn solve(l: usize, rest: &[usize], res: usize) -> bool {
    if rest.is_empty() {
        l == res
    } else {
        let r = rest[0];
        solve(l + r, &rest[1..], res) || solve(l * r, &rest[1..], res)
    }
}

fn part1(input: &Input) -> usize {
    input
        .iter()
        .filter(|(res, nums)| solve(nums[0], &nums[1..], *res))
        .map(|(res, _)| res)
        .sum()
}

fn concat_nums(l: usize, r: usize) -> usize {
    format!("{l}{r}").parse::<usize>().unwrap()
}

fn solve2(l: usize, rest: &[usize], res: usize) -> bool {
    if rest.is_empty() {
        l == res
    } else {
        let r = rest[0];
        solve2(l + r, &rest[1..], res)
            || solve2(l * r, &rest[1..], res)
            || solve2(concat_nums(l, r), &rest[1..], res)
    }
}

fn part2(input: &Input) -> usize {
    input
        .iter()
        .filter(|(res, nums)| solve2(nums[0], &nums[1..], *res))
        .map(|(res, _)| res)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    static INPUT: &'static str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20

        "#;

    #[test]
    fn part1() {
        assert_eq!(super::part1(&parse_input(INPUT)), 3749);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&parse_input(INPUT)), 11387);
    }
}
