pub fn run(input_lines: &[&str]) {
    let input = parse_input(input_lines);

    println!("Part1: {}", part1(input.clone()));
    println!("Part2: {}", part2(input.clone()));
}

fn parse_input<'a>(input_lines: &[&'a str]) -> Vec<&'a str> {
    input_lines.iter().cloned().collect::<Vec<&str>>()
}

type Input<'a> = Vec<&'a str>;

#[derive(Debug)]
struct Directory {
    name: char,
    size: usize,
}

use std::convert::TryFrom;

fn traverse_impl(instructions: &mut Vec<&str>, directories: &mut Vec<Directory>) -> usize {
    let mut sum: usize = 0;

    loop {
        let Some(current_instruction) = instructions.pop() else {
            break;
        };

        if current_instruction.contains("$ ls") {
        } else if current_instruction.contains("$ cd ..") {
            break;
        } else if current_instruction.contains("$ cd") {
            sum += traverse_impl(instructions, directories);
        } else if !current_instruction.contains("dir ") {
            let [size, ..] =
                <[&str; 2]>::try_from(current_instruction.split(' ').collect::<Vec<&str>>())
                    .ok()
                    .unwrap();
            sum += size.parse::<usize>().unwrap();
        }
    }

    directories.push(Directory {
        name: '0',
        size: sum,
    });
    sum
}

fn traverse(instructions: Input) -> Vec<Directory> {
    let mut directories: Vec<Directory> = Vec::new();
    let mut instructions = instructions;
    instructions.reverse();
    let root_directory_size = traverse_impl(&mut instructions, &mut directories);

    directories.push(Directory {
        name: '0',
        size: root_directory_size,
    });

    directories
}

fn part1(instructions: Input) -> usize {
    let directories = traverse(instructions);

    directories.iter().filter(|dir| dir.size <= 100000).map(|dir| dir.size).sum::<usize>() 
}

fn part2(instructions: Input) -> usize {
    let directories = traverse(instructions);

    let root_size = directories.last().unwrap().size;
    let size_to_free = 30000000 - (70000000 - root_size);

    directories.iter().filter(|dir| dir.size >= size_to_free).map(|dir| dir.size).min().unwrap() 
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    const INPUT_LINES: &'static [&'static str] = &[
        "$ cd /",
        "$ ls",
        "dir a",
        "14848514 b.txt",
        "8504156 c.dat",
        "dir d",
        "$ cd a",
        "$ ls",
        "dir e",
        "29116 f",
        "2557 g",
        "62596 h.lst",
        "$ cd e",
        "$ ls",
        "584 i",
        "$ cd ..",
        "$ cd ..",
        "$ cd d",
        "$ ls",
        "4060174 j",
        "8033020 d.log",
        "5626152 d.ext",
        "7214296 k",
    ];

    #[test]
    fn part1() {
        let input = parse_input(&INPUT_LINES);
        let result = super::part1(input.clone());

        assert_eq!(95437, result);
    }

    #[test]
    fn part2() {
        let input = parse_input(&INPUT_LINES);
        let result = super::part2(input.clone());

        assert_eq!(24933642, result);
    }
}
