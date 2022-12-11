use std::collections::VecDeque;

pub fn run(input_lines: &[&str]) {
    let input = parse_input(input_lines);

    println!("Part1:\n{}", part1(input.clone()));
    println!("Part2:\n{}", part2(input.clone()));
}

#[derive(Debug, Clone)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

#[derive(Debug, Clone)]
struct Test {
    divisor: usize,
    monkey_true: usize,
    monkey_false: usize,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    test: Test,
}

fn parse_input(input_lines: &[&str]) -> Vec<Monkey> {
    let monkeys_lines = input_lines.split(|line| line.len() == 0);

    monkeys_lines
        .map(|monkey_lines| {
            let mut monkey_lines = monkey_lines.iter();
            //skip monkey index
            _ = monkey_lines.next();

            //starting items
            let items: VecDeque<usize> = monkey_lines
                .next()
                .unwrap()
                .split(": ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|lvl| lvl.parse::<usize>().unwrap())
                .collect();

            //e.g. "Operation: new = old + 6" -> ["+", "6"]
            let operation_tokens: Vec<&str> = monkey_lines
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .skip(4)
                .collect();

            let operation = match operation_tokens[..] {
                ["*", "old"] => Operation::Square,
                ["+", value] => Operation::Add(value.parse::<usize>().unwrap()),
                ["*", value] => Operation::Multiply(value.parse::<usize>().unwrap()),
                _ => unreachable!(),
            };

            let divisor = monkey_lines
                .next()
                .unwrap()
                .split(" ")
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let monkey_true = monkey_lines
                .next()
                .unwrap()
                .split(" ")
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let monkey_false = monkey_lines
                .next()
                .unwrap()
                .split(" ")
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let test = Test {
                divisor,
                monkey_true,
                monkey_false,
            };

            Monkey {
                items,
                operation,
                test,
            }
        })
        .collect()
}

fn part1(monkeys: Vec<Monkey>) -> usize {
    let mut monkey_inspects = vec![0 as usize; monkeys.len()];
    let mut monkeys = monkeys;

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                let mut worry_lvl = monkeys[i].items.pop_front().unwrap();
                monkey_inspects[i] += 1;
                worry_lvl = match monkeys[i].operation {
                    Operation::Square => worry_lvl * worry_lvl,
                    Operation::Multiply(value) => worry_lvl * value,
                    Operation::Add(value) => worry_lvl + value,
                };

                worry_lvl = worry_lvl / 3;

                let monkey_index = if worry_lvl % monkeys[i].test.divisor != 0 {
                    monkeys[i].test.monkey_false
                } else {
                    monkeys[i].test.monkey_true
                };

                monkeys[monkey_index].items.push_back(worry_lvl);
            }
        }
    }

    monkey_inspects.sort();

    let monkey_inspects = &monkey_inspects[monkey_inspects.len() - 2..];

    monkey_inspects[0] * monkey_inspects[1]
}

fn part2(monkeys: Vec<Monkey>) -> usize {
    let mut monkey_inspects = vec![0 as usize; monkeys.len()];
    let mut monkeys = monkeys;

    let big_divisor = monkeys
        .iter()
        .map(|monkey| monkey.test.divisor)
        .reduce(|a, b| a * b)
        .unwrap();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                let mut worry_lvl = monkeys[i].items.pop_front().unwrap();
                monkey_inspects[i] += 1;

                worry_lvl = match monkeys[i].operation {
                    Operation::Square => worry_lvl * worry_lvl,
                    Operation::Multiply(value) => worry_lvl * value,
                    Operation::Add(value) => worry_lvl + value,
                };

                worry_lvl = worry_lvl % big_divisor;

                let monkey_index = if worry_lvl % monkeys[i].test.divisor != 0 {
                    monkeys[i].test.monkey_false
                } else {
                    monkeys[i].test.monkey_true
                };

                monkeys[monkey_index].items.push_back(worry_lvl);
            }
        }
    }

    monkey_inspects.sort();

    let monkey_inspects = &monkey_inspects[monkey_inspects.len() - 2..];

    monkey_inspects[0] * monkey_inspects[1]
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    const INPUT_LINES: &'static [&'static str] = &[
        "Monkey 0:",
        "  Starting items: 79, 98",
        "  Operation: new = old * 19",
        "  Test: divisible by 23",
        "    If true: throw to monkey 2",
        "    If false: throw to monkey 3",
        "",
        "Monkey 1:",
        "  Starting items: 54, 65, 75, 74",
        "  Operation: new = old + 6",
        "  Test: divisible by 19",
        "    If true: throw to monkey 2",
        "    If false: throw to monkey 0",
        "",
        "Monkey 2:",
        "  Starting items: 79, 60, 97",
        "  Operation: new = old * old",
        "  Test: divisible by 13",
        "    If true: throw to monkey 1",
        "    If false: throw to monkey 3",
        "",
        "Monkey 3:",
        "  Starting items: 74",
        "  Operation: new = old + 3",
        "  Test: divisible by 17",
        "    If true: throw to monkey 0",
        "    If false: throw to monkey 1",
    ];

    #[test]
    fn part1() {
        let input = parse_input(&INPUT_LINES);
        let result = super::part1(input.clone());

        assert_eq!(10605, result);
    }

    #[test]
    fn part2() {
        let input = parse_input(&INPUT_LINES);
        let result = super::part2(input.clone());

        assert_eq!(2713310158, result);
    }
}
