use itertools::Itertools;

pub fn run(input_lines: &[&str]) {
    let input = parse_input(input_lines);

    println!("Part1:\n{}", part1(&input));
    println!("Part2:\n{}", part2(&input));
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn parse_input(input_lines: &[&str]) -> Vec<Instruction> {
    input_lines
        .iter()
        .map(
            |instruction| match instruction.split(' ').collect::<Vec<&str>>()[..] {
                ["addx", value] => Instruction::Addx(value.parse::<i32>().unwrap()),
                ["noop"] => Instruction::Noop,
                _ => unreachable!(),
            },
        )
        .collect::<Vec<Instruction>>()
}

fn part1(instructions: &[Instruction]) -> u32 {
    let mut sum: u32 = 0;
    let mut cycle: u32 = 0;
    let mut x: i32 = 1;

    for instruction in instructions {
        let cycles = match instruction {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        };

        for _ in 0..cycles {
            cycle += 1;

            let sum_cycles = [20, 60, 100, 140, 180, 220];

            if sum_cycles.contains(&cycle) {
                sum += cycle * x as u32;
            }
        }

        if let Instruction::Addx(value) = instruction {
            x += *value;
        }
    }

    sum
}

fn part2(instructions: &[Instruction]) -> String {
    let mut x: i32 = 1;
    let mut pixels: Vec<char> = Vec::new();

    for instruction in instructions {
        let cycles = match instruction {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        };

        for _ in 0..cycles {
            let pixel_pos = pixels.len() as i32 % 40;

            if (x - 1..=x + 1).contains(&pixel_pos) {
                pixels.push('#');
            } else {
                pixels.push('.');
            }
        }

        if let Instruction::Addx(value) = instruction {
            x += *value;
        }
    }

    pixels
        .chunks(40)
        .map(|chars| String::from_iter(chars.iter()))
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    const INPUT_LINES: &'static [&'static str] = &[
        "addx 15", "addx -11", "addx 6", "addx -3", "addx 5", "addx -1", "addx -8", "addx 13",
        "addx 4", "noop", "addx -1", "addx 5", "addx -1", "addx 5", "addx -1", "addx 5", "addx -1",
        "addx 5", "addx -1", "addx -35", "addx 1", "addx 24", "addx -19", "addx 1", "addx 16",
        "addx -11", "noop", "noop", "addx 21", "addx -15", "noop", "noop", "addx -3", "addx 9",
        "addx 1", "addx -3", "addx 8", "addx 1", "addx 5", "noop", "noop", "noop", "noop", "noop",
        "addx -36", "noop", "addx 1", "addx 7", "noop", "noop", "noop", "addx 2", "addx 6", "noop",
        "noop", "noop", "noop", "noop", "addx 1", "noop", "noop", "addx 7", "addx 1", "noop",
        "addx -13", "addx 13", "addx 7", "noop", "addx 1", "addx -33", "noop", "noop", "noop",
        "addx 2", "noop", "noop", "noop", "addx 8", "noop", "addx -1", "addx 2", "addx 1", "noop",
        "addx 17", "addx -9", "addx 1", "addx 1", "addx -3", "addx 11", "noop", "noop", "addx 1",
        "noop", "addx 1", "noop", "noop", "addx -13", "addx -19", "addx 1", "addx 3", "addx 26",
        "addx -30", "addx 12", "addx -1", "addx 3", "addx 1", "noop", "noop", "noop", "addx -9",
        "addx 18", "addx 1", "addx 2", "noop", "noop", "addx 9", "noop", "noop", "noop", "addx -1",
        "addx 2", "addx -37", "addx 1", "addx 3", "noop", "addx 15", "addx -21", "addx 22",
        "addx -6", "addx 1", "noop", "addx 2", "addx 1", "noop", "addx -10", "noop", "noop",
        "addx 20", "addx 1", "addx 2", "addx 2", "addx -6", "addx -11", "noop", "noop", "noop",
    ];

    #[test]
    fn part1() {
        let input = parse_input(&INPUT_LINES);
        let result = super::part1(&input);

        assert_eq!(13140, result);
    }

    #[test]
    fn part2() {
        let input = parse_input(&INPUT_LINES);
        let result = super::part2(&input);

        let output = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        println!("{output}");

        assert_eq!(output, result);
    }
}
