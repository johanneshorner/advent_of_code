use lazy_static::lazy_static;
use std::collections::{hash_map::Entry::*, HashMap};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeKind = 3,
    FullHouse = 4,
    FourKind = 5,
    FiveKind = 6,
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let other = other.clone() as u8;
        (self.clone() as u8).cmp(&other)
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Card(char);

lazy_static! {
    static ref ORDERING: HashMap<char, i32> = HashMap::from([
        ('2', 0),
        ('3', 1),
        ('4', 2),
        ('5', 3),
        ('6', 4),
        ('7', 5),
        ('8', 6),
        ('9', 7),
        ('T', 8),
        ('J', 9),
        ('Q', 10),
        ('K', 11),
        ('A', 12),
    ]);
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        ORDERING[&self.0].cmp(&ORDERING[&other.0])
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    r#type: HandType,
    bid: i32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&self.r#type, &self.cards).cmp(&(&other.r#type, &other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
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
                cards: cards.chars().map(Card).collect::<Vec<_>>(),
                r#type,
                bid: bid.parse::<i32>().unwrap(),
            }
        })
        .collect::<Vec<_>>();
    input
}

fn part1(input: Input) -> Output {
    input
        .iter()
        .sorted()
        .enumerate()
        .map(|(i, hand)| (i as i32 + 1) * hand.bid)
        .sum()
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
