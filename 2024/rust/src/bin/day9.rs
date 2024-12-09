use core::str;
use std::collections::VecDeque;

use itertools::Itertools;

fn main() {
    let input = parse_input(include_str!("../../../input/day9.txt"));
    let input2 = parse_input2(include_str!("../../../input/day9.txt"));
    eprintln!("{}", part1(&input));
    eprintln!("{}", part2(&input2));
}

#[derive(Debug, Clone)]
struct Space {
    blocks: Vec<usize>,
    free: usize,
}

impl Space {
    fn push(&mut self, block: usize) -> Option<usize> {
        if self.free > 0 {
            self.blocks.push(block);
            self.free -= 1;
            None
        } else {
            Some(block)
        }
    }

    fn pop(&mut self) -> Option<usize> {
        if let Some(block) = self.blocks.pop() {
            self.free += 1;
            Some(block)
        } else {
            None
        }
    }
}

type Input = Vec<Space>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .chunks(2)
        .into_iter()
        .enumerate()
        .map(|(i, file)| {
            let (blocks, free) = match file.collect::<Vec<usize>>().as_slice() {
                [blocks, free] => (*blocks as usize, *free as usize),
                [blocks] => (*blocks as usize, 0usize),
                _ => unreachable!("no"),
            };

            Space {
                blocks: vec![i; blocks],
                free,
            }
        })
        .collect()
}

fn part1(input: &Input) -> usize {
    let mut input = input.clone();

    while let Some(mut last) = input.pop() {
        'find_free: for i in 0..input.len() {
            let curr = &mut input[i];
            while let Some(block) = last.pop() {
                if let Some(block) = curr.push(block) {
                    last.push(block);
                    continue 'find_free;
                }
            }
        }
        if last.blocks.len() != 0 {
            input.push(last);
            break;
        }
    }

    input
        .iter()
        .flat_map(|space| &space.blocks)
        .enumerate()
        .map(|(i, id)| i * *id as usize)
        .sum()
}

#[derive(Debug, Clone)]
struct Space2 {
    blocks: Vec<usize>,
    new_blocks: Vec<usize>,
    free_back: usize,
    free_front: usize,
}

type Input2 = Vec<Space2>;

fn parse_input2(input: &str) -> Input2 {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .chunks(2)
        .into_iter()
        .enumerate()
        .map(|(i, file)| {
            let (blocks, free) = match file.collect::<Vec<usize>>().as_slice() {
                [blocks, free] => (*blocks as usize, *free as usize),
                [blocks] => (*blocks as usize, 0usize),
                _ => unreachable!("no"),
            };

            Space2 {
                blocks: vec![i; blocks],
                new_blocks: vec![],
                free_back: free,
                free_front: 0,
            }
        })
        .collect()
}

fn part2(input: &Input2) -> usize {
    let mut input = input.clone();

    let mut not_moved = VecDeque::new();

    while let Some(mut last) = input.pop() {
        if let Some(free_space) = input
            .iter_mut()
            .find(|space| space.free_back >= last.blocks.len())
        {
            last.free_front = last.blocks.len();
            free_space.free_back -= last.blocks.len();
            free_space.new_blocks.extend(last.blocks.drain(..));
        }
        not_moved.push_front(last);
    }

    not_moved
        .iter()
        .flat_map(|space| {
            std::iter::repeat_n(&0, space.free_front)
                .chain(space.blocks.iter())
                .chain(space.new_blocks.iter())
                .chain(std::iter::repeat_n(&0, space.free_back))
                .cloned()
                .collect::<Vec<usize>>()
        })
        .enumerate()
        .map(|(i, id)| i * id as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{parse_input, parse_input2};

    static INPUT: &'static str = r#"2333133121414131402"#;

    #[test]
    fn part1() {
        assert_eq!(super::part1(&parse_input(INPUT)), 1928);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&parse_input2(INPUT)), 2858);
    }
}
