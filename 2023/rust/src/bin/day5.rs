use std::ops::Range;

#[derive(Debug, Clone)]
struct Map {
    source: i32,
    dest: i32,
    range: i32,
}

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<i32>,
    maps: Vec<Vec<Map>>,
}

type Input = Almanac;
type Output = i32;

fn parse_input(input: &str) -> Input {
    let mut input = input.trim().split("\n\n");
    let seeds = input
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let maps = input
        .map(|map| {
            map.split('\n')
                .skip(1)
                .map(|map_numbers| {
                    let [dest, source, range] = &map_numbers
                        .split_whitespace()
                        .map(|num| num.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()[..]
                    else {
                        unreachable!("bad input")
                    };
                    Map {
                        source: *source,
                        dest: *dest,
                        range: *range,
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    Almanac { seeds, maps }
}

fn part1(input: Input) -> Output {
    println!("{input:?}");

    input
        .seeds
        .into_iter()
        .map(|seed| {
            input.maps.iter().fold(seed, |acc, maps| {
                println!("{acc} {maps:?}");
                if let Some(dest) = maps.iter().find_map(|map| {
                    if (map.source..map.source + map.range).contains(&acc) {
                        Some((map.source - seed) + map.dest)
                    } else {
                        None
                    }
                }) {
                    dest
                } else {
                    acc
                }
            })
        })
        .min()
        .unwrap()
}

fn part2(input: Input) -> Output {
    1
}

fn main() {
    let input = parse_input(include_str!("../../../input/day1.in"));
    println!("Part 1: {}", part1(input.clone()));
    // println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    const PART1_SOLUTION: Output = 35;
    const PART2_SOLUTION: Output = 1;

    #[test]
    fn test_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(input), PART1_SOLUTION);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(input), PART2_SOLUTION);
    }
}
