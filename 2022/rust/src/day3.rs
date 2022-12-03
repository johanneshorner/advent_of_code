pub fn run(input_lines: &[&str]) {
    let input = parse_input(input_lines);

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn parse_input(input_lines: &[&str]) -> Vec<Vec<u8>> {
    input_lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| {
                    let as_num = c as u8;
                    if c.is_uppercase() {
                        (as_num - 'A' as u8) + 27
                    } else {
                        (as_num - 'a' as u8) + 1
                    }
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
}

use std::collections::HashSet;

fn part1(backpacks: &[Vec<u8>]) -> usize {
    let mut sum = 0;

    for backpack in backpacks {
        let compartments = backpack.split_at(backpack.len() / 2);
        let unique_items: HashSet<&u8> = HashSet::from_iter(compartments.0.iter());

        for priority in compartments.1 {
            if unique_items.contains(priority) {
                sum += *priority as usize;
                break;
            }
        }
    }

    sum
}

use std::collections::HashMap;

fn part2(backpacks: &[Vec<u8>]) -> usize {
    // let groups = backpacks.chunks(3);
    //
    // let mut sum = 0;
    //
    // for group in groups {
    //     // get unique items
    //     let group: Vec<HashSet<&u8>> = group
    //         .iter()
    //         .map(|backpack| backpack.iter().collect::<HashSet<&u8>>())
    //         .collect();
    //
    //     let mut unique_items: HashMap<u8, usize> = HashMap::new();
    //
    //     for backpack in group {
    //         for priority in backpack {
    //             unique_items
    //                 .entry(*priority)
    //                 .and_modify(|count| *count += 1)
    //                 .or_insert(1);
    //         }
    //     }
    //
    //     let (priority, _) =unique_items.iter().find(|(_, count)| **count == 3).unwrap();
    //     sum += *priority as usize;
    // }
    //
    // sum

    backpacks.chunks(3).map(|group| {
        *group
            .iter()
            .map(|backpack| backpack.iter().collect::<HashSet<&u8>>())
            .reduce(|a, b| a.intersection(&b).cloned().collect::<HashSet<&u8>>())
            .unwrap()
            .iter()
            .cloned()
            .next()
            .unwrap() as usize
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    const INPUT_LINES: &'static [&'static str] = &[
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ];

    #[test]
    fn part1() {
        let input = parse_input(&INPUT_LINES);
        let result = super::part1(&input);

        assert_eq!(157, result);
    }

    #[test]
    fn part2() {
        let input = parse_input(&INPUT_LINES);
        let result = super::part2(&input);

        assert_eq!(70, result);
    }
}
