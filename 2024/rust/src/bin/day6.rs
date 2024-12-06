use core::str;
use std::collections::HashSet;

fn main() {
    let input = parse_input(include_str!("../../../input/day6.txt"));
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

fn sim(grid: &Vec<Vec<u8>>, mut current_pos: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut visited = HashSet::new();
    visited.insert(current_pos);

    'outer: for direction in ['^', '>', 'v', '<'].iter().cycle() {
        let (cur_x, cur_y) = current_pos;
        match direction {
            '^' => {
                let mut not_found = true;
                for y in (0..cur_y).rev() {
                    if grid[y][cur_x] == b'#' {
                        current_pos = (cur_x, y + 1);
                        not_found = false;
                        break;
                    } else {
                        visited.insert((cur_x, y));
                    }
                }
                if not_found {
                    break 'outer;
                }
            }
            '>' => {
                let mut not_found = true;
                for x in cur_x + 1..grid[cur_y].len() {
                    if grid[cur_y][x] == b'#' {
                        current_pos = (x - 1, cur_y);
                        not_found = false;
                        break;
                    } else {
                        visited.insert((x, cur_y));
                    }
                }
                if not_found {
                    break 'outer;
                }
            }
            'v' => {
                let mut not_found = true;
                for y in cur_y + 1..grid.len() {
                    if grid[y][cur_x] == b'#' {
                        current_pos = (cur_x, y - 1);
                        not_found = false;
                        break;
                    } else {
                        visited.insert((cur_x, y));
                    }
                }
                if not_found {
                    break 'outer;
                }
            }
            '<' => {
                let mut not_found = true;
                for x in (0..cur_x).rev() {
                    if grid[cur_y][x] == b'#' {
                        current_pos = (x + 1, cur_y);
                        not_found = false;
                        break;
                    } else {
                        visited.insert((x, cur_y));
                    }
                }
                if not_found {
                    break 'outer;
                }
            }
            _ => unreachable!(""),
        }
    }

    visited
}

fn sim_obstruction(grid: &Vec<Vec<u8>>, mut current_pos: (usize, usize)) -> bool {
    let mut visited = HashSet::new();
    {
        let (x, y) = current_pos;
        visited.insert((x, y, '^'));
    }

    for direction in ['^', '>', 'v', '<'].iter().cycle() {
        let (cur_x, cur_y) = current_pos;
        match direction {
            '^' => {
                let mut not_found = true;
                for y in (0..cur_y).rev() {
                    if visited.get(&(cur_x, y, *direction)).is_some() {
                        return true;
                    } else if grid[y][cur_x] == b'#' {
                        current_pos = (cur_x, y + 1);
                        not_found = false;
                        break;
                    } else {
                        visited.insert((cur_x, y, *direction));
                    }
                }
                if not_found {
                    return false;
                }
            }
            '>' => {
                let mut not_found = true;
                for x in cur_x + 1..grid[cur_y].len() {
                    if visited.get(&(x, cur_y, *direction)).is_some() {
                        return true;
                    } else if grid[cur_y][x] == b'#' {
                        current_pos = (x - 1, cur_y);
                        not_found = false;
                        break;
                    } else {
                        visited.insert((x, cur_y, *direction));
                    }
                }
                if not_found {
                    return false;
                }
            }
            'v' => {
                let mut not_found = true;
                for y in cur_y + 1..grid.len() {
                    if visited.get(&(cur_x, y, *direction)).is_some() {
                        return true;
                    } else if grid[y][cur_x] == b'#' {
                        current_pos = (cur_x, y - 1);
                        not_found = false;
                        break;
                    } else {
                        visited.insert((cur_x, y, *direction));
                    }
                }
                if not_found {
                    return false;
                }
            }
            '<' => {
                let mut not_found = true;
                for x in (0..cur_x).rev() {
                    if visited.get(&(x, cur_y, *direction)).is_some() {
                        return true;
                    } else if grid[cur_y][x] == b'#' {
                        current_pos = (x + 1, cur_y);
                        not_found = false;
                        break;
                    } else {
                        visited.insert((x, cur_y, *direction));
                    }
                }
                if not_found {
                    return false;
                }
            }
            _ => unreachable!(""),
        }
    }

    unreachable!("nop")
}

fn part1(input: &Input) -> usize {
    let mut current_pos = (0, 0);

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == b'^' {
                current_pos = (x, y);
                break;
            }
        }
    }

    sim(input, current_pos).len()
}

fn part2(input: &Input) -> usize {
    let mut current_pos = (0, 0);

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == b'^' {
                current_pos = (x, y);
                break;
            }
        }
    }

    let mut visited = sim(input, current_pos);
    visited.remove(&current_pos);

    let mut count = 0;
    for pos in visited {
        let mut input = input.clone();
        input[pos.1][pos.0] = b'#';
        if sim_obstruction(&input, current_pos) {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    static INPUT: &'static str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
        "#;

    #[test]
    fn part1() {
        assert_eq!(super::part1(&parse_input(INPUT)), 41);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&parse_input(INPUT)), 6);
    }
}
