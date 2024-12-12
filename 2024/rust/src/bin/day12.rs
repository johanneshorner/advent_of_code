#![feature(ascii_char)]
#![feature(ascii_char_variants)]
use core::str;
use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use rust::util::grid::{Grid, Point};

fn main() {
    let input = parse_input(include_str!("../../../input/day12.txt"));
    eprintln!("{}", part1(&input));
    eprintln!("{}", part2(&input));
}

type Input = Grid<u8>;

fn parse_input(input: &str) -> Input {
    Grid::new(
        input
            .trim()
            .lines()
            .map(|line| line.bytes().collect::<Vec<u8>>())
            .collect(),
    )
}

fn bfs(grid: &Grid<u8>, plots_visited: &mut HashSet<Point>, start: Point, plant_type: u8) -> usize {
    let mut queue = VecDeque::from([start]);
    let mut perimeter_visited = HashSet::new();

    let (mut area, mut perimeter) = (1, 0);
    while let Some(current) = queue.pop_front() {
        for (x, y) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let neighbour = Point {
                x: current.x + x,
                y: current.y + y,
            };
            if let Some(neighbour_plot) = grid.get(neighbour.x, neighbour.y) {
                if *neighbour_plot == plant_type {
                    if plots_visited.insert(neighbour) {
                        area += 1;
                        queue.push_back(neighbour);
                    }
                } else if perimeter_visited.insert((neighbour, (x, y))) {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }
        }
    }

    area * perimeter
}

fn part1(grid: &Input) -> usize {
    let mut price = 0;
    for plant_type in grid.flattened().into_iter().unique() {
        let mut plots_visited = HashSet::new();
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                if plant_type == grid.get(x as isize, y as isize).unwrap() {
                    let start = Point {
                        x: x as isize,
                        y: y as isize,
                    };
                    if !plots_visited.insert(start) {
                        continue;
                    }
                    price += bfs(grid, &mut plots_visited, start, *plant_type);
                }
            }
        }
    }

    price
}

fn bfs2(
    grid: &Grid<u8>,
    plots_visited: &mut HashSet<Point>,
    start: Point,
    plant_type: u8,
) -> usize {
    let mut queue = VecDeque::from([start]);
    let mut boundary_edges = HashSet::new();

    let mut area = 1;
    while let Some(current) = queue.pop_front() {
        for (x, y) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let neighbour = Point {
                x: current.x + x,
                y: current.y + y,
            };
            if let Some(neighbour_plot) = grid.get(neighbour.x, neighbour.y) {
                if *neighbour_plot == plant_type {
                    if plots_visited.insert(neighbour) {
                        area += 1;
                        queue.push_back(neighbour);
                    }
                } else {
                    boundary_edges.insert((current, neighbour));
                }
            } else {
                boundary_edges.insert((current, neighbour));
            }
        }
    }

    let mut distinct_sides = 0;
    for &(Point { x: x1, y: y1 }, Point { x: x2, y: y2 }) in &boundary_edges {
        if x1 == x2 {
            if !boundary_edges.contains(&(Point { x: x1 - 1, y: y1 }, Point { x: x1 - 1, y: y2 })) {
                distinct_sides += 1;
            }
        } else if y1 == y2 {
            if !boundary_edges.contains(&(Point { x: x1, y: y1 - 1 }, Point { x: x2, y: y1 - 1 })) {
                distinct_sides += 1;
            }
        }
    }

    area * distinct_sides
}

fn part2(grid: &Input) -> usize {
    let mut price = 0;
    for plant_type in grid.flattened().into_iter().unique() {
        let mut plots_visited = HashSet::new();
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                if plant_type == grid.get(x as isize, y as isize).unwrap() {
                    let start = Point {
                        x: x as isize,
                        y: y as isize,
                    };
                    if !plots_visited.insert(start) {
                        continue;
                    }
                    price += bfs2(grid, &mut plots_visited, start, *plant_type);
                }
            }
        }
    }

    price
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    static INPUT: &'static str = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
        "#;

    #[test]
    fn part1() {
        assert_eq!(super::part1(&parse_input(INPUT)), 1930);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&parse_input(INPUT)), 1206);
    }
}
