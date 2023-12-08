use std::collections::HashMap;

use num::Integer;

#[derive(Debug, Clone)]
struct Node {
    left: String,
    right: String,
}

type Input = (Vec<char>, HashMap<String, Node>);
type Output = i64;

fn parse_input(input: &str) -> Input {
    let directions = input
        .lines()
        .clone()
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<_>>();
    let map: HashMap<String, Node> = input
        .replace("(", "")
        .replace(")", "")
        .lines()
        .skip(2)
        .map(|line| {
            let (key, rest) = line.split_once(" = ").unwrap();
            let (left, right) = rest.split_once(", ").unwrap();
            (
                key.into(),
                Node {
                    left: left.into(),
                    right: right.into(),
                },
            )
        })
        .collect();

    (directions, map)
}

fn part1(input: Input) -> Output {
    let (directions, map) = input;

    let mut current_node = &map["AAA"];
    let mut step_count = 0;
    for direction in directions.iter().cycle() {
        step_count += 1;
        let next_node = match direction {
            'L' => &current_node.left,
            'R' => &current_node.right,
            _ => unreachable!("bad input"),
        };

        if next_node == "ZZZ" {
            break;
        }

        current_node = &map[next_node];
    }

    step_count
}

fn part2(input: Input) -> Output {
    let (directions, map) = input;

    let start_nodes = map
        .iter()
        .filter(|(key, _)| key.ends_with('A'))
        .map(|(_, value)| value)
        .collect::<Vec<_>>();

    let mut counts = Vec::new();

    for start_node in start_nodes {
        let mut current_node = start_node;
        let mut step_count = 0;
        for direction in directions.iter().cycle() {
            step_count += 1;
            let next_node = match direction {
                'L' => &current_node.left,
                'R' => &current_node.right,
                _ => unreachable!("bad input"),
            };

            if next_node.ends_with('Z') {
                break;
            }

            current_node = &map[next_node];
        }
        counts.push(step_count);
    }
    counts.into_iter().reduce(|acc, e| acc.lcm(&e)).unwrap()
}

fn main() {
    let input = parse_input(include_str!("../../../input/day8.in"));
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    const INPUT2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    const PART1_SOLUTION: Output = 6;
    const PART2_SOLUTION: Output = 6;

    #[test]
    fn test_part1() {
        let input = parse_input(INPUT1);
        assert_eq!(part1(input), PART1_SOLUTION);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(INPUT2);
        assert_eq!(part2(input), PART2_SOLUTION);
    }
}
