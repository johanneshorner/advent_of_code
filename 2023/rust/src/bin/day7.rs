use std::collections::{hash_map::Entry::*, HashMap};

use itertools::Itertools;

#[derive(Debug, Clone)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<char>,
    r#type: HandType,
    bid: i32,
}

type Input = Vec<Hand>;
type Output = i32;

fn parse_input(input: &str) -> Input {
    let input = input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();

            let mut map = HashMap::new();

            for card in cards.chars() {
                match map.entry(card) {
                    Occupied(mut entry) => *entry.get_mut() += 1,
                    Vacant(entry) => _ = entry.insert(1),
                }
            }

            use HandType::*;
            let r#type = match map.values().sorted_by(|a, b| b.cmp(a)).collect::<Vec<_>>()[..] {
                [5] => FiveKind,
                [4, _] => FourKind,
                [3, 2] => FullHouse,
                [3, _, _] => ThreeKind,
                [2, 2, _] => TwoPair,
                [2, _, _, _] => OnePair,
                _ => HighCard,
            };

            Hand {
                cards: cards.chars().collect::<Vec<_>>(),
                r#type,
                bid: bid.parse::<i32>().unwrap(),
            }
        })
        .collect::<Vec<_>>();
    input
}

fn part1(input: Input) -> Output {
    println!("{input:?}");
    0
}

fn part2(input: Input) -> Output {
    1
}

fn main() {
    let input = parse_input(include_str!("../../../input/day7.in"));
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    const PART1_SOLUTION: Output = 6440;
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
