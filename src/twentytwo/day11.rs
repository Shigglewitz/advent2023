use crate::create_advent_day;

create_advent_day!("2022", "11");

fn part1_with_input(input: &str) -> u64 {
    let mut biz = MonkeyBusiness::parse(&input, 3);
    for _ in 0..20 {
        biz.round();
    }
    return biz.monkey_business();
}

fn part2_with_input(input: &str) -> u64 {
    let mut biz = MonkeyBusiness::parse(&input, 0);
    for _ in 0..10_000 {
        biz.round();
    }
    return biz.monkey_business();
}

enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    fn execute(&self, input: u64) -> u64 {
        match self {
            Operation::Add(add_me) => input + add_me,
            Operation::Multiply(mult_me) => input * mult_me,
            Operation::Square => input * input,
        }
    }
}

enum NormalizeOperation {
    DivideByThree,
    ModuloBy(u64),
}

impl NormalizeOperation {
    fn execute(&self, input: u64) -> u64 {
        match self {
            NormalizeOperation::DivideByThree => input / 3,
            NormalizeOperation::ModuloBy(modulo) => input % modulo,
        }
    }
}

struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test_divisor: u64,
    true_monkey_index: usize,
    false_monkey_index: usize,
    num_inspections: u64,
}

impl Monkey {
    fn parse(input: &[&str]) -> Self {
        let (_, all_items) = input[1].split_once(": ").unwrap();
        let items = all_items
            .split(", ")
            .map(|str| str.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let operation_input = &input[2].trim().split(" ").collect::<Vec<_>>()[3..6];
        let operation = match (operation_input[0], operation_input[1], operation_input[2]) {
            ("old", "*", "old") => Operation::Square,
            (_, "+", parse_me) => {
                let add = parse_me.parse::<u64>().unwrap();
                Operation::Add(add)
            }
            (_, "*", parse_me) => {
                let multiply = parse_me.parse::<u64>().unwrap();
                Operation::Multiply(multiply)
            }
            _ => unreachable!("unexpected function"),
        };
        let test_divisor = input[3].split(" ").last().unwrap().parse::<u64>().unwrap();
        let true_monkey_index = input[4]
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let false_monkey_index = input[5]
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        return Self {
            items,
            operation,
            test_divisor,
            true_monkey_index,
            false_monkey_index,
            num_inspections: 0,
        };
    }

    fn throw_items(&mut self, normalizer: &NormalizeOperation) -> Vec<(usize, u64)> {
        let throws = self
            .items
            .iter()
            .map(|&item| {
                let raw_worry = self.operation.execute(item);
                let new_worry = normalizer.execute(raw_worry);
                let new_index = if new_worry % self.test_divisor == 0 {
                    self.true_monkey_index
                } else {
                    self.false_monkey_index
                };
                (new_index, new_worry)
            })
            .collect::<Vec<_>>();
        self.num_inspections += self.items.len() as u64;
        self.items.clear();
        return throws;
    }
}

struct MonkeyBusiness {
    monkeys: Vec<Monkey>,
    normalize: NormalizeOperation,
}

impl MonkeyBusiness {
    fn parse(input: &str, normalizer_input: u64) -> Self {
        let monkeys = input
            .lines()
            .collect::<Vec<_>>()
            .chunks(7)
            .map(Monkey::parse)
            .collect::<Vec<_>>();
        let normalize = if normalizer_input != 0 {
            NormalizeOperation::DivideByThree
        } else {
            let modulo = monkeys.iter().map(|monkey| monkey.test_divisor).product();
            NormalizeOperation::ModuloBy(modulo)
        };
        return Self { monkeys, normalize };
    }

    fn round(&mut self) {
        let num_monkeys = self.monkeys.len();
        for monkey_index in 0..num_monkeys {
            let throws = self.monkeys[monkey_index].throw_items(&self.normalize);
            for throw in throws {
                self.monkeys[throw.0].items.push(throw.1);
            }
        }
    }

    fn monkey_business(&self) -> u64 {
        let mut inspections = self
            .monkeys
            .iter()
            .map(|monkey| monkey.num_inspections)
            .collect::<Vec<_>>();
        inspections.sort();
        return inspections.iter().rev().take(2).product();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = &create("test.txt").solve_part1();

        assert_eq!(actual, "10605");
    }

    #[test]
    fn part2_works() {
        let actual = &create("test.txt").solve_part2();

        assert_eq!(actual, "2713310158");
    }
}
