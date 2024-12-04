use std::collections::HashSet;

fn main() {
    let input = parse_input(include_str!("../../../input/day4.txt"));
    eprintln!("{}", part1(&input));
    eprintln!("{}", part2(&input));
}

type Input = Vec<Vec<char>>;

fn parse_input(input: &str) -> Input {
    let mut input: Input = input
        .trim()
        .split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let padding_chars: Vec<char> = ".".repeat(input[0].len() + 2).chars().collect();

    for i in 0..input.len() {
        input[i].insert(0, '.');
        input[i].push('.');
    }

    input.insert(0, padding_chars.clone());
    input.push(padding_chars);

    input
}

fn part1(input: &Input) -> usize {
    let mut count = 0;

    for y in 1..input.len() - 1 {
        for x in 1..input[0].len() - 1 {
            if input[y][x] == 'X'
                && input[y][x + 1] == 'M'
                && input[y][x + 2] == 'A'
                && input[y][x + 3] == 'S'
            {
                count += 1;
            }
            if input[y][x] == 'X'
                && input[y + 1][x + 1] == 'M'
                && input[y + 2][x + 2] == 'A'
                && input[y + 3][x + 3] == 'S'
            {
                count += 1;
            }
            if input[y][x] == 'X'
                && input[y + 1][x] == 'M'
                && input[y + 2][x] == 'A'
                && input[y + 3][x] == 'S'
            {
                count += 1;
            }
            if input[y][x] == 'X'
                && input[y + 1][x - 1] == 'M'
                && input[y + 2][x - 2] == 'A'
                && input[y + 3][x - 3] == 'S'
            {
                count += 1;
            }
            if input[y][x] == 'X'
                && input[y][x - 1] == 'M'
                && input[y][x - 2] == 'A'
                && input[y][x - 3] == 'S'
            {
                count += 1;
            }
            if input[y][x] == 'X'
                && input[y - 1][x - 1] == 'M'
                && input[y - 2][x - 2] == 'A'
                && input[y - 3][x - 3] == 'S'
            {
                count += 1;
            }
            if input[y][x] == 'X'
                && input[y - 1][x] == 'M'
                && input[y - 2][x] == 'A'
                && input[y - 3][x] == 'S'
            {
                count += 1;
            }
            if input[y][x] == 'X'
                && input[y - 1][x + 1] == 'M'
                && input[y - 2][x + 2] == 'A'
                && input[y - 3][x + 3] == 'S'
            {
                count += 1;
            }
        }
    }

    count
}

fn part2(input: &Input) -> usize {
    let mut count = 0;
    let mut mas_a_positions = HashSet::new();

    for y in 1..input.len() - 1 {
        for x in 1..input[0].len() - 1 {
            if input[y][x] == 'M' && input[y + 1][x + 1] == 'A' && input[y + 2][x + 2] == 'S' {
                let pos = (y + 1, x + 1);
                if !mas_a_positions.insert(pos) {
                    count += 1;
                }
            }
            if input[y][x] == 'M' && input[y + 1][x - 1] == 'A' && input[y + 2][x - 2] == 'S' {
                let pos = (y + 1, x - 1);
                if !mas_a_positions.insert(pos) {
                    count += 1;
                }
            }
            if input[y][x] == 'M' && input[y - 1][x - 1] == 'A' && input[y - 2][x - 2] == 'S' {
                let pos = (y - 1, x - 1);
                if !mas_a_positions.insert(pos) {
                    count += 1;
                }
            }
            if input[y][x] == 'M' && input[y - 1][x + 1] == 'A' && input[y - 2][x + 2] == 'S' {
                let pos = (y - 1, x + 1);
                if !mas_a_positions.insert(pos) {
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    static INPUT: &'static str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
        "#;

    #[test]
    fn part1() {
        assert_eq!(super::part1(&parse_input(INPUT)), 18);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&parse_input(INPUT)), 9);
    }
}
