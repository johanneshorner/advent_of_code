pub fn run(input_lines: &[&str]) {
    let input = parse_input(input_lines);

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[derive(Copy, Clone, Debug)]
struct Area {
    start: usize,
    end: usize,
}

fn parse_input(input_lines: &[&str]) -> Vec<(Area, Area)> {
    input_lines
        .iter()
        .map(|line| {
            let areas = line
                .split(",")
                .map(|area| {
                    let area = area
                        .split("-")
                        .map(|num| num.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();
                    Area {
                        start: area[0],
                        end: area[1],
                    }
                })
                .collect::<Vec<Area>>();
            (areas[0], areas[1])
        })
        .collect::<Vec<(Area, Area)>>()
}

fn contains(area_1: &Area, area_2: &Area) -> bool {
    area_1.start <= area_2.start && area_1.end >= area_2.end
}

fn part1(pairs: &[(Area, Area)]) -> usize {
    let mut sum: usize = 0;
    for pair in pairs {
        let (elve_1, elve_2) = pair;
        if contains(elve_1, elve_2) || contains(elve_2, elve_1) {
            sum += 1;
        }
    }

    sum
}

use std::collections::HashSet;

fn overlap(area_1: &Area, area_2: &Area) -> bool {
    let set_1: HashSet<usize> = (area_1.start..(area_1.end + 1)).collect();
    let set_2: HashSet<usize> = (area_2.start..(area_2.end + 1)).collect();

    set_1.intersection(&set_2).count() > 0
}

fn part2(pairs: &[(Area, Area)]) -> usize {
    let mut sum: usize = 0;
    for pair in pairs {
        let (elve_1, elve_2) = pair;
        if overlap(elve_1, elve_2) || overlap(elve_2, elve_1) {
            sum += 1;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    const INPUT_LINES: &'static [&'static str] = &[
        "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
    ];

    #[test]
    fn part1() {
        let input = parse_input(&INPUT_LINES);
        let result = super::part1(&input);

        assert_eq!(2, result);
    }

    #[test]
    fn part2() {
        let input = parse_input(&INPUT_LINES);
        let result = super::part2(&input);

        assert_eq!(4, result);
    }
}
