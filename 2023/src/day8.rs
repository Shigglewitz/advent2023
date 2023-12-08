use std::collections::HashMap;

use crate::utils;
use num::integer;

pub fn part1(file_name: &str) -> i64 {
    let input = utils::read_file("day8", file_name);

    let problem = Problem::parse(&input);
    return problem.solve_zzz("AAA");
}

pub fn part2(file_name: &str) -> i64 {
    let input = utils::read_file("day8", file_name);

    let problem = Problem::parse(&input);
    let current_nodes: Vec<String> = problem
        .nodes
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(|value| value.to_string())
        .collect();
    let solutions = current_nodes
        .iter()
        .map(|node| problem.solve_ends_with_z(node));
    return solutions.fold(1, |a, b| integer::lcm(a, b));
}

struct Problem {
    instructions: Vec<char>,
    nodes: HashMap<String, (String, String)>,
}

impl Problem {
    fn parse(input: &str) -> Problem {
        let mut lines = input.lines();

        let instructions: Vec<char> = lines.next().unwrap().chars().collect();
        let mut nodes: HashMap<String, (String, String)> = HashMap::new();
        lines.skip(1).for_each(|line| {
            let mut split = line.split(" = ");
            let key = split.next().unwrap().to_string();
            let mut pair = split
                .next()
                .unwrap()
                .strip_prefix("(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split(", ");
            nodes.insert(
                key,
                (
                    pair.next().unwrap().to_string(),
                    pair.next().unwrap().to_string(),
                ),
            );
        });

        return Problem {
            instructions,
            nodes,
        };
    }

    fn solve_zzz(&self, key: &str) -> i64 {
        let num_instructions = self.instructions.len();
        let mut num_steps = 0;
        let mut next_key = key;
        while next_key != &"ZZZ".to_string() {
            let instruction = self.instructions[num_steps % num_instructions];
            if instruction == 'L' {
                next_key = &self.nodes.get(next_key).unwrap().0;
            } else {
                next_key = &self.nodes.get(next_key).unwrap().1;
            }
            num_steps = num_steps + 1;
        }

        return num_steps as i64;
    }

    fn solve_ends_with_z(&self, key: &str) -> i64 {
        let num_instructions = self.instructions.len();
        let mut num_steps = 0;
        let mut next_key = key;
        while !next_key.ends_with("Z") {
            let instruction = self.instructions[num_steps % num_instructions];
            if instruction == 'L' {
                next_key = &self.nodes.get(next_key).unwrap().0;
            } else {
                next_key = &self.nodes.get(next_key).unwrap().1;
            }
            num_steps = num_steps + 1;
        }

        return num_steps as i64;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = part1("test.txt");

        assert_eq!(actual, 2);
    }

    #[test]
    fn part1_second_input_works() {
        let actual = part1("test2.txt");

        assert_eq!(actual, 6);
    }

    #[test]
    fn part2_works() {
        let actual = part2("test3.txt");

        assert_eq!(actual, 6);
    }

    #[test]
    fn problem_parse_works() {
        let actual = Problem::parse(&utils::read_file("day8", "test.txt"));

        assert_eq!(actual.instructions, vec!('R', 'L'));
    }
}
