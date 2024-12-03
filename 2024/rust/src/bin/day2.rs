use itertools::*;

fn main() {
    let input = parse_input(include_str!("../../../input/day2.txt"));
    eprintln!("{}", part1(&input));
    eprintln!("{}", part2(&input));
}

type Input = Vec<Vec<u32>>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split("\n")
        .map(|line| {
            line.split(" ")
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn part1(reports: &Input) -> usize {
    reports.iter().filter(|report| is_safe(*report)).count()
}

fn is_safe(report: &Vec<u32>) -> bool {
    if !report.is_sorted_by(|a, b| a < b) && !report.is_sorted_by(|a, b| a > b) {
        false
    } else {
        for i in 0..report.len() - 1 {
            let prev = report[i];
            let next = report[i + 1];
            let diff = prev.abs_diff(next);
            if diff == 0 || diff > 3 {
                return false;
            }
        }

        true
    }
}

fn gen_almost_safe_variants(report: &Vec<u32>) -> Vec<Vec<u32>> {
    std::iter::once(report)
        .cycle()
        .take(report.len())
        .cloned()
        .enumerate()
        .map(|(i, mut variant)| {
            _ = variant.remove(i);
            variant
        })
        .collect::<Vec<Vec<u32>>>()
}

fn part2(reports: &Input) -> usize {
    reports
        .iter()
        .filter(|report| is_safe(report) || gen_almost_safe_variants(*report).iter().any(is_safe))
        .count()
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    static INPUT: &'static str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
        "#;

    #[test]
    fn part1() {
        assert_eq!(super::part1(&parse_input(INPUT)), 2);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&parse_input(INPUT)), 4);
    }
}
