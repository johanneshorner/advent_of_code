pub fn run(input_lines: &[&str]) {
    let input = parse_input(input_lines);

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn parse_input(input_lines: &[&str]) -> Vec<Vec<usize>> {
    input_lines
        .split(|line| line.len() == 0)
        .map(|calories_group| {
            calories_group
                .iter()
                .map(|calories| calories.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

fn part1(calories_groups: &[Vec<usize>]) -> usize {
    calories_groups
        .iter()
        .map(|calories_group| calories_group.iter().sum::<usize>())
        .max()
        .unwrap()
}

fn part2(calories_groups: &[Vec<usize>]) -> usize {
    let mut calories: Vec<usize> = calories_groups
        .iter()
        .map(|calories_group| calories_group.iter().sum::<usize>()).collect();

    calories.sort();

    calories[(calories.len() - 3)..].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    #[test]
    fn part1() {
        let input_lines = vec![
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ];

        let input = parse_input(&input_lines);
        let result = super::part1(&input);

        assert_eq!(24000, result);
    }

    #[test]
    fn part2() {
        let input_lines = vec![
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ];

        let input = parse_input(&input_lines);
        let result = super::part2(&input);

        assert_eq!(45000, result);
    }
}
