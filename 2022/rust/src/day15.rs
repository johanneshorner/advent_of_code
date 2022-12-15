pub fn run(input_lines: &[&str]) {
    let input = parse_input(input_lines);

    println!("Part1:\n{}", part1(&input, 2_000_000));
    println!("Part2:\n{}", part2(&input, 4_000_000));
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Sensor {
    pos: Point,
    beacon_pos: Point,
}

use regex::Regex;

fn parse_input(input_lines: &[&str]) -> Vec<Sensor> {
    let re = Regex::new(r"[-]*\d+").unwrap();

    input_lines
        .iter()
        .map(|line| {
            let [x, y, x_beacon, y_beacon,] = re.captures_iter(line)
                .map(|capture| capture.get(0).unwrap().as_str().parse::<isize>().unwrap())
                .collect::<Vec<_>>()[..] else {
                    panic!("wrong input");
                };
            Sensor {
                pos: Point { x, y },
                beacon_pos: Point {
                    x: x_beacon,
                    y: y_beacon,
                },
            }
        })
        .collect::<Vec<_>>()
}

use std::collections::HashSet;

fn manhattan_distance(point1: &Point, point2: &Point) -> usize {
    point1.x.abs_diff(point2.x) + point1.y.abs_diff(point2.y)
}

fn part1(sensors: &[Sensor], row: isize) -> usize {
    let mut non_beacon_positions = HashSet::new();

    for sensor in sensors {
        let max_distance = manhattan_distance(&sensor.pos, &sensor.beacon_pos);

        for x in sensor.pos.x - max_distance as isize..=sensor.pos.x + max_distance as isize {
            // we only need to add points that are in the row we are looking for
            let point = Point { x, y: row };
            if manhattan_distance(&point, &sensor.pos) <= max_distance {
                non_beacon_positions.insert(point);
            }
        }
    }

    non_beacon_positions
        .iter()
        .filter(|pos| pos.y == row)
        .count()
        - 1
}

fn part2(sensors: &[Sensor], max_pos: usize) -> usize {
    let mut non_beacon_positions = HashSet::new();

    for (i, sensor) in sensors.iter().enumerate() {
        let max_distance = manhattan_distance(&sensor.pos, &sensor.beacon_pos);

        let left_edge = sensor.pos.x - max_distance as isize;
        let right_edge = sensor.pos.x + max_distance as isize;
        let top_edge = sensor.pos.y - max_distance as isize;
        let bottom_edge = sensor.pos.y + max_distance as isize;

        let x_range = if left_edge <= max_pos as isize && left_edge >= 0 {
            left_edge as usize..max_pos
        } else if right_edge <= max_pos as isize && right_edge >= 0 {
            0..right_edge as usize
        } else {
            0..0
        };

        let y_range = if top_edge <= max_pos as isize && top_edge >= 0 {
            top_edge as usize..max_pos
        } else if bottom_edge <= max_pos as isize && bottom_edge >= 0 {
            0..bottom_edge as usize
        } else {
            0..0
        };

        println!("ranges: {:?} {:?}", x_range, y_range);

        for x in x_range {
            for y in y_range.clone() {
                let point = Point {
                    x: x as isize,
                    y: y as isize,
                };
                if manhattan_distance(&point, &sensor.pos) <= max_distance {
                    non_beacon_positions.insert(point);
                }
            }
        }
        println!("{i}");
    }

    for x in 0..max_pos {
        for y in 0..max_pos {
            println!("{x}, {y}");
            if !non_beacon_positions.contains(&Point {
                x: x as isize,
                y: y as isize,
            }) {
                println!("found {x}, {y}");
                return x as usize * 4_000_000 + y as usize;
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    const INPUT_LINES: &'static [&'static str] = &[
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
        "Sensor at x=9, y=16: closest beacon is at x=10, y=16",
        "Sensor at x=13, y=2: closest beacon is at x=15, y=3",
        "Sensor at x=12, y=14: closest beacon is at x=10, y=16",
        "Sensor at x=10, y=20: closest beacon is at x=10, y=16",
        "Sensor at x=14, y=17: closest beacon is at x=10, y=16",
        "Sensor at x=8, y=7: closest beacon is at x=2, y=10",
        "Sensor at x=2, y=0: closest beacon is at x=2, y=10",
        "Sensor at x=0, y=11: closest beacon is at x=2, y=10",
        "Sensor at x=20, y=14: closest beacon is at x=25, y=17",
        "Sensor at x=17, y=20: closest beacon is at x=21, y=22",
        "Sensor at x=16, y=7: closest beacon is at x=15, y=3",
        "Sensor at x=14, y=3: closest beacon is at x=15, y=3",
        "Sensor at x=20, y=1: closest beacon is at x=15, y=3;",
    ];

    #[test]
    fn part1() {
        let input = parse_input(&INPUT_LINES);
        let result = super::part1(&input, 10);

        assert_eq!(26, result);
    }

    #[test]
    fn part2() {
        let input = parse_input(&INPUT_LINES);
        let result = super::part2(&input, 20);

        assert_eq!(56000011, result);
    }
}
