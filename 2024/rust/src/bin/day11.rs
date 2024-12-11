use core::str;
use std::collections::HashMap;

fn main() {
    let input = parse_input(include_str!("../../../input/day11.txt"));
    eprintln!("{}", part1(&input));
}

type Input = Vec<usize>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Stone {
    n: usize,
    generation: usize,
}

static GENERATIONS: usize = 25;

fn blink(stone: Stone, finished_evolutions: &mut HashMap<Stone, usize>) -> usize {
    if stone.generation == GENERATIONS {
        return 1;
    }
    if let Some(count) = finished_evolutions.get(&stone) {
        return *count;
    }

    let count = if stone.n == 0 {
        let count = blink(
            Stone {
                n: 1,
                generation: stone.generation + 1,
            },
            finished_evolutions,
        );
        count
    } else {
        let n_str = stone.n.to_string();
        let len = n_str.chars().count();
        if len % 2 == 0 {
            let (l, r) = n_str.split_at(len / 2);
            let (l, r) = (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap());
            let count = blink(
                Stone {
                    n: l,
                    generation: stone.generation + 1,
                },
                finished_evolutions,
            ) + blink(
                Stone {
                    n: r,
                    generation: stone.generation + 1,
                },
                finished_evolutions,
            );
            count
        } else {
            let count = blink(
                Stone {
                    n: stone.n * 2024,
                    generation: stone.generation + 1,
                },
                finished_evolutions,
            );
            count
        }
    };

    finished_evolutions.insert(stone, count);
    count
}

fn part1(input: &Input) -> usize {
    let mut finished_evolutions = HashMap::new();
    input
        .iter()
        .map(|n| {
            blink(
                Stone {
                    n: *n,
                    generation: 0,
                },
                &mut finished_evolutions,
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    static INPUT: &'static str = r#"125 17"#;

    #[test]
    fn part1() {
        assert_eq!(super::part1(&parse_input(INPUT)), 55312);
    }
}
