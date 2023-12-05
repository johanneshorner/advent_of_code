#![feature(result_option_inspect)]

use std::ops::Range;

#[derive(Debug, Clone)]
struct Map {
    source: i64,
    dest: i64,
    range: i64,
}

enum MatchType {
    Complete(Range<i64>),
    Bigger {
        mapped: Range<i64>,
        rest: Vec<Range<i64>>,
    },
    Partly {
        mapped: Range<i64>,
        rest: Range<i64>,
    },
    Not(Range<i64>),
}

use MatchType::*;

impl Map {
    fn apply(&self, range: &Range<i64>) -> MatchType {
        let map_range = self.source..self.source + self.range;

        if map_range.contains(&range.start) && map_range.contains(&(range.end - 1)) {
            Complete((range.start - self.source) + self.dest..(range.end - self.source) + self.dest)
        } else if range.start < self.source && range.end > self.source + self.range {
            Bigger {
                mapped: self.dest..self.dest + self.range,
                rest: vec![
                    range.start..self.source,
                    self.source + self.range..range.end,
                ],
            }
        } else if range.start < self.source && map_range.contains(&(range.end - 1)) {
            Partly {
                mapped: self.dest..(range.end - self.source) + self.dest,
                rest: range.start..self.source,
            }
        } else if map_range.contains(&range.start) && range.end > self.source + self.range {
            Partly {
                mapped: (range.start - self.source) + self.dest..self.dest + self.range,
                rest: self.source + self.range..range.end,
            }
        } else {
            Not(range.clone())
        }
    }
}

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<Range<i64>>,
    maps: Vec<Vec<Map>>,
}

type Input = Almanac;
type Output = i64;

// fn parse_input(input: &str) -> Input {
//     let mut input = input.trim().split("\n\n");
//     let seeds = input
//         .next()
//         .unwrap()
//         .split_once(": ")
//         .unwrap()
//         .1
//         .split_whitespace()
//         .map(|num| num.parse::<i64>().unwrap())
//         .collect::<Vec<i64>>();
//
//     let maps = input
//         .map(|map| {
//             map.split('\n')
//                 .skip(1)
//                 .map(|map_numbers| {
//                     let [dest, source, range] = &map_numbers
//                         .split_whitespace()
//                         .map(|num| num.parse::<i64>().unwrap())
//                         .collect::<Vec<i64>>()[..]
//                     else {
//                         unreachable!("bad input")
//                     };
//                     Map {
//                         source: *source,
//                         dest: *dest,
//                         range: *range,
//                     }
//                 })
//                 .collect::<Vec<_>>()
//         })
//         .collect::<Vec<Vec<_>>>();
//
//     Almanac { seeds, maps }
// }

fn parse_input_2(input: &str) -> Input {
    let mut input = input.trim().split("\n\n");
    let seeds_old = input
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut seeds = Vec::new();

    let mut i = 0;
    while i < seeds_old.len() {
        seeds.push(seeds_old[i]..seeds_old[i] + seeds_old[i + 1]);
        i = i + 2;
    }

    let maps = input
        .map(|map| {
            map.split('\n')
                .skip(1)
                .map(|map_numbers| {
                    let [dest, source, range] = &map_numbers
                        .split_whitespace()
                        .map(|num| num.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>()[..]
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

// fn part1(input: Input) -> Output {
//     input
//         .seeds
//         .into_iter()
//         .map(|seed| {
//             input.maps.iter().fold(seed, |acc, maps| {
//                 if let Some(dest) = maps.iter().find_map(|map| {
//                     if (map.source..map.source + map.range).contains(&acc) {
//                         Some((acc - map.source) + map.dest)
//                     } else {
//                         None
//                     }
//                 }) {
//                     dest
//                 } else {
//                     acc
//                 }
//             })
//         })
//         .min()
//         .unwrap()
// }

fn part2(input: Input) -> Output {
    input
        .seeds
        .into_iter()
        .map(|seed_range| {
            let seed_ranges = vec![seed_range];
            let end_ranges = input.maps.iter().fold(seed_ranges, |acc, maps| {
                let mut unmapped_seed_ranges = acc;
                let mut mapped_seed_ranges = Vec::new();

                for map in maps {
                    let mut temp = Vec::new();
                    for seed_range in &unmapped_seed_ranges {
                        match map.apply(seed_range) {
                            Complete(range) => mapped_seed_ranges.push(range),
                            Bigger { mapped, rest } => {
                                mapped_seed_ranges.push(mapped);
                                temp.extend_from_slice(&rest);
                            }
                            Partly { mapped, rest } => {
                                mapped_seed_ranges.push(mapped);
                                temp.push(rest);
                            }
                            Not(range) => temp.push(range),
                        }
                    }
                    unmapped_seed_ranges = temp;
                }

                mapped_seed_ranges.append(&mut unmapped_seed_ranges);

                mapped_seed_ranges
            });

            end_ranges
                .iter()
                .min_by_key(|range| range.start)
                .unwrap()
                .start
        })
        .min()
        .unwrap()
}

fn main() {
    // let input = parse_input(include_str!("../../../input/day5.in"));
    // println!("Part 1: {}", part1(input.clone()));
    let input = parse_input_2(include_str!("../../../input/day5.in"));
    println!("Part 2: {}", part2(input));
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
    const PART2_SOLUTION: Output = 46;

    // #[test]
    // fn test_part1() {
    //     let input = parse_input(INPUT);
    //     assert_eq!(part1(input), PART1_SOLUTION);
    // }

    #[test]
    fn test_part2() {
        let input = parse_input_2(INPUT);
        assert_eq!(part2(input), PART2_SOLUTION);
    }
}
