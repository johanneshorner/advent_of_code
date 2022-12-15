pub fn run(input_lines: &[&str]) {
    let input = parse_input(input_lines);

    println!("Part1:\n{}", part1(&input, 2_000_000));
    println!("Part2:\n{}", part2(&input, 2_000_000));
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
                beacon_pos: Point { x: x_beacon, y: y_beacon },
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

        // non_beacon_positions.extend((sensor.pos.x..sensor.pos.x + max_distance as isize).map(|x| Point { x, y: sensor.pos.y }));
    }

    non_beacon_positions.iter().filter(|pos| pos.y == row).count()
}

fn part2(sensors: &[Sensor], row: isize) -> usize {
    0
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
        let result = super::part2(&input, 10);

        assert_eq!(45000, result);
    }
}
