use core::str;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let input = parse_input(include_str!("../../../input/day8.txt"));
    eprintln!("{}", part1(&input));
    eprintln!("{}", part2(&input));
}

type Input = Vec<Vec<u8>>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect()
}

type Point = (isize, isize);

fn antinode_positions(l: &Point, r: &Point) -> (Point, Point) {
    let diff_x = l.0 - r.0;
    let diff_y = l.1 - r.1;

    ((l.0 + diff_x, l.1 + diff_y), (r.0 - diff_x, r.1 - diff_y))
}

fn gen_antinodes(antennas: &HashSet<Point>) -> HashSet<Point> {
    let mut iter = antennas.iter();
    let mut antinodes = HashSet::new();

    while let Some(antenna) = iter.next() {
        for other in iter.clone() {
            let (l, r) = antinode_positions(antenna, other);
            antinodes.insert(l);
            antinodes.insert(r);
        }
    }

    antinodes
}

fn part1(input: &Input) -> usize {
    let mut frequencies: HashMap<u8, HashSet<Point>> = HashMap::new();

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let pos = input[y][x];
            if pos != b'.' {
                let frequency = frequencies.entry(pos).or_default();
                frequency.insert((x as isize, y as isize));
            }
        }
    }

    frequencies
        .values()
        .flat_map(|antennas| gen_antinodes(antennas))
        .filter(|(x, y)| {
            *x >= 0 && *y >= 0 && (*x as usize) < input[0].len() && (*y as usize) < input.len()
        })
        .unique()
        .count()
}

fn antinode_positions2(l: &Point, r: &Point, x_max: usize, y_max: usize) -> HashSet<Point> {
    let diff_x = l.0 - r.0;
    let diff_y = l.1 - r.1;

    let mut antinodes = HashSet::new();

    let mut base_x = l.0;
    let mut base_y = l.1;
    antinodes.insert((base_x, base_y));
    loop {
        base_x += diff_x;
        base_y += diff_y;
        if base_x >= 0 && (base_x as usize) <= x_max && base_y >= 0 && (base_y as usize) <= y_max {
            antinodes.insert((base_x, base_y));
        } else {
            break;
        }
    }

    let mut base_x = r.0;
    let mut base_y = r.1;
    antinodes.insert((base_x, base_y));
    loop {
        base_x -= diff_x;
        base_y -= diff_y;
        if base_x >= 0 && (base_x as usize) <= x_max && base_y >= 0 && (base_y as usize) <= y_max {
            antinodes.insert((base_x, base_y));
        } else {
            break;
        }
    }

    antinodes
}

fn gen_antinodes2(antennas: &HashSet<Point>, x_max: usize, y_max: usize) -> HashSet<Point> {
    let mut iter = antennas.iter();
    let mut antinodes = HashSet::new();

    while let Some(antenna) = iter.next() {
        for other in iter.clone() {
            let new_antinodes = antinode_positions2(antenna, other, x_max, y_max);
            antinodes.extend(new_antinodes.iter());
        }
    }

    antinodes
}

fn part2(input: &Input) -> usize {
    let mut frequencies: HashMap<u8, HashSet<Point>> = HashMap::new();

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let pos = input[y][x];
            if pos != b'.' {
                let frequency = frequencies.entry(pos).or_default();
                frequency.insert((x as isize, y as isize));
            }
        }
    }

    frequencies
        .values()
        .flat_map(|antennas| gen_antinodes2(antennas, input[0].len() - 1, input.len() - 1))
        .unique()
        .count()
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    static INPUT: &'static str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
        "#;

    #[test]
    fn part1() {
        assert_eq!(super::part1(&parse_input(INPUT)), 14);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&parse_input(INPUT)), 34);
    }
}
