pub fn run(input_lines: &[&str]) {
    let input = parse_input(input_lines);

    println!("Part1:\n{}", part1(input.clone()));
    println!("Part2:\n{}", part2(input.clone()));
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

use std::collections::hash_set::HashSet;

fn parse_input(input_lines: &[&str]) -> HashSet<Point> {
    input_lines
        .iter()
        .map(|line| {
            let rocks_coords: Vec<Point> = line
                .split(" -> ")
                .map(|coords_str| {
                    let coords: Vec<isize> = coords_str
                        .split(',')
                        .map(|num| num.parse::<isize>().unwrap())
                        .collect();
                    Point {
                        x: coords[0],
                        y: coords[1],
                    }
                })
                .collect();

            let mut rock_points: HashSet<Point> = HashSet::new();

            let mut rocks_coords = rocks_coords.iter().peekable();
            while let Some(rock_coords) = rocks_coords.next() {
                let Some(next_coords) = rocks_coords.peek() else {
                    break;
                };

                if rock_coords.x == next_coords.x {
                    let y_range = if (rock_coords.y as isize - next_coords.y as isize) < 0 {
                        rock_coords.y..=next_coords.y
                    } else {
                        next_coords.y..=rock_coords.y
                    };

                    rock_points.extend(y_range.map(|y| Point {
                        x: rock_coords.x,
                        y,
                    }));
                } else {
                    let x_range = if (rock_coords.x as isize - next_coords.x as isize) < 0 {
                        rock_coords.x..=next_coords.x
                    } else {
                        next_coords.x..=rock_coords.x
                    };

                    rock_points.extend(x_range.map(|x| Point {
                        x,
                        y: rock_coords.y,
                    }));
                }
            }

            rock_points
        })
        .flatten()
        .collect::<HashSet<Point>>()
}

fn part1(mut points: HashSet<Point>) -> usize {
    let highest_y = points
        .iter()
        .max_by(|point1, point2| point1.y.cmp(&point2.y))
        .unwrap()
        .y;

    let point_count = points.len();

    'outer: loop {
        let mut current_sand = Point { x: 500, y: 0 };

        loop {
            // sand falls out of map
            if current_sand.y == highest_y {
                break 'outer;
            }

            match current_sand {
                Point { x, y } if !points.contains(&Point { x, y: y + 1 }) => current_sand.y += 1,
                Point { x, y } if !points.contains(&Point { x: x - 1, y: y + 1 }) => {
                    current_sand.x -= 1;
                    current_sand.y += 1;
                }
                Point { x, y } if !points.contains(&Point { x: x + 1, y: y + 1 }) => {
                    current_sand.x += 1;
                    current_sand.y += 1;
                }
                Point { x, y } => {
                    _ = points.insert(Point { x, y });
                    break;
                }
            }
        }
    }

    points.len() - point_count
}

fn part2(mut points: HashSet<Point>) -> usize {
    let highest_y = points
        .iter()
        .max_by(|point1, point2| point1.y.cmp(&point2.y))
        .unwrap()
        .y;

    let point_count = points.len();

    loop {
        let mut current_sand = Point { x: 500, y: 0 };

        loop {
            match current_sand {
                Point { x, y } if y + 1 == highest_y + 2 => {
                    _ = points.insert(Point { x, y });
                    break;
                }
                Point { x, y } if !points.contains(&Point { x, y: y + 1 }) => current_sand.y += 1,
                Point { x, y } if !points.contains(&Point { x: x - 1, y: y + 1 }) => {
                    current_sand.x -= 1;
                    current_sand.y += 1;
                }
                Point { x, y } if !points.contains(&Point { x: x + 1, y: y + 1 }) => {
                    current_sand.x += 1;
                    current_sand.y += 1;
                }
                Point { x, y } => {
                    _ = points.insert(Point { x, y });
                    break;
                }
            }
        }

        if points.contains(&Point { x: 500, y: 0 }) {
            break;
        }
    }

    points.len() - point_count
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    const INPUT_LINES: &'static [&'static str] = &[
        "498,4 -> 498,6 -> 496,6",
        "503,4 -> 502,4 -> 502,9 -> 494,9",
    ];

    #[test]
    fn part1() {
        let input = parse_input(&INPUT_LINES);
        let result = super::part1(input);

        assert_eq!(24, result);
    }

    #[test]
    fn part2() {
        let input = parse_input(&INPUT_LINES);
        let result = super::part2(input);

        assert_eq!(93, result);
    }
}
