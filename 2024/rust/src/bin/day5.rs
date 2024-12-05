use itertools::*;
use std::{cmp::Ordering, collections::HashSet};

fn main() {
    let input = parse_input(include_str!("../../../input/day5.txt"));
    eprintln!("{}", part1(&input));
    eprintln!("{}", part2(&input));
}

type Input = (HashSet<(u32, u32)>, Vec<Vec<u32>>);

fn parse_input(input: &str) -> Input {
    let (ordering_rules, updates) = input.trim().split_once("\n\n").unwrap();

    let ordering_rules: HashSet<(u32, u32)> =
        HashSet::from_iter(ordering_rules.split("\n").map(|line| {
            let (l, r) = line.split_once('|').unwrap();
            (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap())
        }));

    let updates: Vec<Vec<u32>> = updates
        .split("\n")
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    (ordering_rules, updates)
}

fn part1((ref ordering_rules, ref updates): &Input) -> usize {
    updates
        .iter()
        .filter(|pages| pages.is_sorted_by(|a, b| ordering_rules.get(&(*a, *b)).is_some()))
        .map(|pages| pages[pages.len() / 2])
        .sum::<u32>() as usize
}

fn part2((ref ordering_rules, ref updates): &Input) -> usize {
    updates
        .iter()
        .filter(|pages| !pages.is_sorted_by(|a, b| ordering_rules.get(&(*a, *b)).is_some()))
        .map(|pages| {
            pages
                .iter()
                .sorted_by(|a, b| {
                    if ordering_rules.get(&(**a, **b)).is_some() {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                })
                .nth(pages.len() / 2)
                .unwrap()
        })
        .sum::<u32>() as usize
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    static INPUT: &'static str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
        "#;

    #[test]
    fn part1() {
        assert_eq!(super::part1(&parse_input(INPUT)), 143);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&parse_input(INPUT)), 123);
    }
}
