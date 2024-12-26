use std::collections::HashMap;

use crate::create_advent_day;

create_advent_day!("2024", "24");

fn part1_with_input(input: &str) -> i64 {
    let mut split = input.split("\n\n");
    let wires_input = split.next().unwrap();
    let gates_input = split.next().unwrap();
    let mut wires: HashMap<String, u8> = HashMap::new();
    wires_input.lines().for_each(|line| {
        let mut split = line.split_whitespace();
        let label = split.next().unwrap().trim_matches(':').to_owned();
        let value = split.next().unwrap().parse::<u8>().unwrap();
        wires.insert(label, value);
    });
    let mut gates = gates_input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let input1 = split.next().unwrap().to_owned();
            let operation = split.next().unwrap().chars().next().unwrap();
            let input2 = split.next().unwrap().to_owned();
            split.next(); // drop ->
            let output_wire = split.next().unwrap().to_owned();
            Gate {
                input1,
                input2,
                operation,
                output_wire,
            }
        })
        .collect::<Vec<Gate>>();

    loop {
        let (solved, unsolved): (Vec<_>, Vec<_>) = gates
            .into_iter()
            .map(|gate| {
                let solution = gate.solve(&wires);
                (gate, solution)
            })
            .partition(|tuple| tuple.1.is_some());
        solved.iter().for_each(|(gate, solution)| {
            wires.insert(gate.output_wire.clone(), solution.unwrap());
        });
        gates = unsolved
            .into_iter()
            .map(|(gate, _)| gate)
            .collect::<Vec<_>>();
        if gates.len() == 0 {
            break;
        }
    }

    let mut zeds = wires
        .into_iter()
        .filter(|(key, _)| key.starts_with('z'))
        .collect::<Vec<_>>();
    zeds.sort_by(|first, second| second.0.cmp(&first.0));
    let value = zeds
        .iter()
        .map(|(_, value)| value.to_string())
        .collect::<String>();

    return i64::from_str_radix(&value, 2).unwrap();
}

struct Gate {
    input1: String,
    input2: String,
    operation: char,
    output_wire: String,
}

impl Gate {
    fn solve(&self, wires: &HashMap<String, u8>) -> Option<u8> {
        let first = wires.get(&self.input1);
        let second = wires.get(&self.input2);
        if first.is_none() {
            return None;
        }
        if second.is_none() {
            return None;
        }
        let first = first.unwrap();
        let second = second.unwrap();

        let value = match self.operation {
            'A' => *first == 1 && *second == 1,
            'O' => *first == 1 || *second == 1,
            'X' => *first != *second,
            _ => todo!(),
        };
        if value {
            Some(1)
        } else {
            Some(0)
        }
    }
}

fn part2_with_input(_input: &str) -> i64 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!("2024", &actual);
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!("0", &actual);
    }
}
