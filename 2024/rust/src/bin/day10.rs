use core::str;
use std::collections::HashSet;

use rust::util::grid::{Grid, Point};

fn main() {
    let input = parse_input(include_str!("../../../input/day10.txt"));
    eprintln!("{}", part1(&input));
    eprintln!("{}", part2(&input));
}

type Input = Grid<u32>;

fn parse_input(input: &str) -> Input {
    Grid::new(
        input
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|n| n.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect(),
    )
}

fn count_highest_point_paths(grid: &Grid<u32>, start: &Point) -> usize {
    let mut neighbours = vec![start.clone()];
    let mut visited = HashSet::new();

    let mut count = 0;
    while let Some(current) = neighbours.pop() {
        visited.insert(current.clone());
        if let Some(9) = grid.get(current.x, current.y) {
            count += 1;
            continue;
        }

        neighbours.extend(
            grid.neighbour_coords(current.x, current.y)
                .into_iter()
                .filter(|neighbour| {
                    *grid.get_unchecked(neighbour.x, neighbour.y)
                        == grid.get_unchecked(current.x, current.y) + 1
                        && !visited.contains(neighbour)
                }),
        );
    }

    count
}

fn part1(input: &Input) -> usize {
    let mut sum = 0;

    for y in 0..input.height() {
        for x in 0..input.width() {
            if *input.get(x, y).unwrap() == 0 {
                sum += count_highest_point_paths(input, &Point { x, y });
            }
        }
    }

    sum
}

fn count_highest_point_paths2(grid: &Grid<u32>, start: &Point) -> usize {
    let mut neighbours = vec![start.clone()];

    let mut count = 0;
    while let Some(current) = neighbours.pop() {
        if let Some(9) = grid.get(current.x, current.y) {
            count += 1;
            continue;
        }

        neighbours.extend(
            grid.neighbour_coords(current.x, current.y)
                .into_iter()
                .filter(|neighbour| {
                    *grid.get_unchecked(neighbour.x, neighbour.y)
                        == grid.get_unchecked(current.x, current.y) + 1
                }),
        );
    }

    count
}

fn part2(input: &Input) -> usize {
    let mut sum = 0;

    for y in 0..input.height() {
        for x in 0..input.width() {
            if *input.get(x, y).unwrap() == 0 {
                sum += count_highest_point_paths2(input, &Point { x, y });
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    static INPUT: &'static str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#;

    #[test]
    fn part1() {
        assert_eq!(super::part1(&parse_input(INPUT)), 36);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&parse_input(INPUT)), 81);
    }
}
