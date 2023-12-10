type Input = Vec<Vec<char>>;
type Output = i64;

fn parse_input(input: &str) -> Input {
    let mut map = input
        .lines()
        .map(|line| format!(".{line}.").chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let row_length = map[0].len();
    map.insert(0, vec!['.'; row_length]);
    map.push(vec!['.'; row_length]);

    map
}

#[derive(Debug, Clone, Copy)]
enum Origin {
    North,
    East,
    South,
    West,
}

use std::collections::HashSet;

use Origin::*;

fn part1(input: Input) -> Output {
    let mut next_tile = ((0, 0), North);

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == 'S' {
                // start = (x, y);
                next_tile = if ['|', '7', 'F'].contains(&input[y - 1][x]) {
                    ((x, y - 1), South)
                } else if ['|', 'L', 'J'].contains(&input[y + 1][x]) {
                    ((x, y + 1), North)
                } else if ['-', 'L', 'F'].contains(&input[y][x - 1]) {
                    ((x - 1, y), East)
                } else if ['-', 'J', '7'].contains(&input[y][x - 1]) {
                    ((x + 1, y), West)
                } else {
                    unreachable!("no connection from start")
                };
            }
        }
    }
    // println!("\n");
    // for y in 0..input.len() {
    //     println!("{}", input[y].iter().collect::<String>())
    // }
    // println!("\n");

    let mut count = 0;

    loop {
        count += 1;
        let ((x, y), origin) = next_tile;
        next_tile = match (input[y][x], origin) {
            ('|', North) => ((x, y + 1), North),
            ('|', South) => ((x, y - 1), South),
            ('-', East) => ((x - 1, y), East),
            ('-', West) => ((x + 1, y), West),
            ('L', North) => ((x + 1, y), West),
            ('L', East) => ((x, y - 1), South),
            ('J', North) => ((x - 1, y), East),
            ('J', West) => ((x, y - 1), South),
            ('7', South) => ((x - 1, y), East),
            ('7', West) => ((x, y + 1), North),
            ('F', South) => ((x + 1, y), West),
            ('F', East) => ((x, y + 1), North),
            ('S', _) => break,
            _ => unreachable!("unknown character"),
        };
    }

    count / 2
}

fn part2(input: Input) -> Output {
    let mut next_tile = ((0, 0), North);

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == 'S' {
                // start = (x, y);
                next_tile = if ['|', '7', 'F'].contains(&input[y - 1][x]) {
                    ((x, y - 1), South)
                } else if ['|', 'L', 'J'].contains(&input[y + 1][x]) {
                    ((x, y + 1), North)
                } else if ['-', 'L', 'F'].contains(&input[y][x - 1]) {
                    ((x - 1, y), East)
                } else if ['-', 'J', '7'].contains(&input[y][x - 1]) {
                    ((x + 1, y), West)
                } else {
                    unreachable!("no connection from start")
                };
            }
        }
    }
    // println!("\n");
    // for y in 0..input.len() {
    //     println!("{}", input[y].iter().collect::<String>())
    // }
    // println!("\n");
    //

    let mut loop_tiles = HashSet::new();

    loop {
        let ((x, y), origin) = next_tile;
        loop_tiles.insert((x, y));
        next_tile = match (input[y][x], origin) {
            ('|', North) => ((x, y + 1), North),
            ('|', South) => ((x, y - 1), South),
            ('-', East) => ((x - 1, y), East),
            ('-', West) => ((x + 1, y), West),
            ('L', North) => ((x + 1, y), West),
            ('L', East) => ((x, y - 1), South),
            ('J', North) => ((x - 1, y), East),
            ('J', West) => ((x, y - 1), South),
            ('7', South) => ((x - 1, y), East),
            ('7', West) => ((x, y + 1), North),
            ('F', South) => ((x + 1, y), West),
            ('F', East) => ((x, y + 1), North),
            ('S', _) => break,
            _ => unreachable!("unknown character"),
        };
    }

    0
}

fn main() {
    let input = parse_input(include_str!("../../../input/day10.in"));
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
    const INPUT2: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    const PART1_SOLUTION: Output = 4;
    const PART2_SOLUTION: Output = 10;

    #[test]
    fn test_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(input), PART1_SOLUTION);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(INPUT2);
        assert_eq!(part2(input), PART2_SOLUTION);
    }
}
