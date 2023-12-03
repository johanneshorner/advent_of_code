#[derive(Debug, Clone)]
struct Number {
    value: String,
    x: usize,
    y: usize,
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_digit(10)
}

impl Number {
    fn is_part_number(&self, schematic: &Vec<Vec<char>>) -> bool {
        println!("{self:?}");

        let ret = is_symbol(schematic[self.y][self.x - 1])
            || is_symbol(schematic[self.y][self.x + self.value.len()])
            || schematic[self.y - 1][self.x - 1..self.x + self.value.len() + 1]
                .iter()
                .any(|c| is_symbol(*c))
            || schematic[self.y + 1][self.x - 1..self.x + self.value.len() + 1]
                .iter()
                .any(|c| is_symbol(*c));

        println!("ret: {ret}");

        ret
    }

    fn find_gear_coords(&self, schematic: &Vec<Vec<char>>) -> Option<(usize, usize)> {
        println!("{self:?}");

        if schematic[self.y][self.x - 1] == '*' {
            return Some((self.x - 1, self.y));
        } else if schematic[self.y][self.x + self.value.len()] == '*' {
            return Some((self.x + self.value.len(), self.y));
        } else {
            for x in self.x - 1..self.x + self.value.len() + 1 {
                if schematic[self.y - 1][x] == '*' {
                    return Some((x, self.y - 1));
                }
                if schematic[self.y + 1][x] == '*' {
                    return Some((x, self.y + 1));
                }
            }
        }

        None
    }
}

type Input = (Vec<Number>, Vec<Vec<char>>);
type Output = u32;

fn parse_input(input: &str) -> Input {
    let mut schematic = input
        .trim()
        .split('\n')
        .map(|line| format!(".{line}.").chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let row_length = schematic[0].len();
    schematic.insert(0, vec!['.'; row_length]);
    schematic.push(vec!['.'; row_length]);

    let mut numbers = Vec::new();

    for y in 0..schematic.len() {
        let mut x = 0;
        while x < schematic[y].len() {
            if schematic[y][x].is_digit(10) {
                let end = schematic[y][x..]
                    .iter()
                    .position(|c| !c.is_digit(10))
                    .unwrap();
                let value: String = schematic[y][x..x + end].iter().collect();
                let number = Number { value, x, y };
                x = x + number.value.len() + 1;
                numbers.push(number);
            } else {
                x = x + 1;
            }
        }
    }

    (numbers, schematic)
}

fn part1(input: Input) -> Output {
    let mut sum = 0;

    for number in input.0 {
        if number.is_part_number(&input.1) {
            sum = sum + number.value.parse::<u32>().unwrap();
        }
    }

    sum
}

fn part2(input: Input) -> Output {
    let mut sum = 0;

    let mut gear_numbers = Vec::new();

    for number in input.0 {
        if let Some(gear) = number.find_gear_coords(&input.1) {
            gear_numbers.push((number.clone(), gear));
        }
    }

    for i in 0..gear_numbers.len() - 1 {
        let (number, (x, y)) = &gear_numbers[i];

        if let Some(number2) = gear_numbers[i + 1..].iter().find(|gear_number| {
            let (_, (x2, y2)) = gear_number;
            x == x2 && y == y2
        }) {
            let gear_ratio =
                number.value.parse::<u32>().unwrap() * number2.0.value.parse::<u32>().unwrap();
            sum = sum + gear_ratio;
        }
    }

    sum
}

fn main() {
    let input = parse_input(include_str!("../../../input/day3.in"));
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_INPUT: &str = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
            ";
    const PART2_INPUT: &str = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
        ";
    const PART1_SOLUTION: u32 = 4361;
    const PART2_SOLUTION: u32 = 467835;

    #[test]
    fn test_part1() {
        let input = parse_input(PART1_INPUT);
        assert_eq!(part1(input), PART1_SOLUTION);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(PART2_INPUT);
        assert_eq!(part2(input), PART2_SOLUTION);
    }
}
