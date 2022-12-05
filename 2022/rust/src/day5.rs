pub fn run(input_lines: &[&str]) {
    let input = parse_input(input_lines);

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[derive(Debug)]
struct Move {
    count: usize,
    src: usize,
    dest: usize,
}

fn parse_input(input_lines: &[&str]) -> (Vec<Vec<char>>, Vec<Move>) {
    let mut input_lines = input_lines.split(|lines| lines.len() == 0);

    let containers_lines = input_lines.next().unwrap();
    let moves_lines = input_lines.next().unwrap();

    let mut containers_lines = containers_lines
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    containers_lines.reverse();

    let mut containers = Vec::new();

    for i in 0..containers_lines[0].len() {
        let mut stack = Vec::new();
        for line in &containers_lines {
            let c = line.chars().nth(i).unwrap();
            if c != '0' {
                stack.push(c);
            }
        }
        containers.push(stack);
    }

    let moves = moves_lines
        .iter()
        .map(|line| {
            let line = line.replace("move ", "");
            let line = line.replace("from ", "");
            let line = line.replace("to ", "");
            let numbers = line
                .split(" ")
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            Move {
                count: numbers[0],
                src: numbers[1] - 1,
                dest: numbers[2] - 1,
            }
        })
        .collect::<Vec<Move>>();

    (containers, moves)
}

fn part1(input: &(Vec<Vec<char>>, Vec<Move>)) -> String {
    let mut containers = input.0.clone();
    let moves = &input.1;

    for Move { count, src, dest } in moves {
        for _ in 0..*count {
            let container = containers[*src].pop().unwrap();
            containers[*dest].push(container);
        }
    }

    let mut result = String::new();

    for stack in containers {
        result += &stack.last().unwrap().to_string();
    }

    result
}

fn part2(input: &(Vec<Vec<char>>, Vec<Move>)) -> String {
    let mut containers = input.0.clone();
    let moves = &input.1;

    for Move { count, src, dest } in moves {
        let number_of_containers_to_move = containers[*src].len();
        let containers_to_move = containers[*src].drain((number_of_containers_to_move - count)..).collect::<Vec<char>>();
        containers[*dest].extend(containers_to_move);
    }

    let mut result = String::new();

    for stack in containers {
        result += &stack.last().unwrap().to_string();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    const INPUT_LINES: &'static [&'static str] = &[
        "0D0",
        "NC0",
        "ZMP",
        "",
        "move 1 from 2 to 1",
        "move 3 from 1 to 3",
        "move 2 from 2 to 1",
        "move 1 from 1 to 2",
    ];

    #[test]
    fn part1() {
        let input = parse_input(&INPUT_LINES);
        let result = super::part1(&input);

        assert_eq!("CMZ", result);
    }

    #[test]
    fn part2() {
        let input = parse_input(&INPUT_LINES);
        let result = super::part2(&input);

        assert_eq!("MCD", result);
    }
}
