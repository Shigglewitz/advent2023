use std::collections::HashMap;

use crate::create_advent_day;
use num::integer;

create_advent_day!("08");

fn part1_with_input(input: &str) -> i64 {
    let problem = Problem::parse(&input);
    return problem.solve("AAA", "ZZZ");
}

fn part2_with_input(input: &str) -> i64 {
    let problem = Problem::parse(&input);
    let current_nodes: Vec<String> = problem
        .nodes
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(|value| value.to_string())
        .collect();
    let solutions = current_nodes.iter().map(|node| problem.solve(node, "Z"));
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

    fn solve(&self, key: &str, suffix: &str) -> i64 {
        let num_instructions = self.instructions.len();
        let mut num_steps = 0;
        let mut next_key = key;
        while !next_key.ends_with(suffix) {
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
        let actual = create("test.txt").solve_part1();

        assert_eq!(&actual, "2");
    }

    #[test]
    fn part1_second_input_works() {
        let actual = create("test2.txt").solve_part1();

        assert_eq!(&actual, "6");
    }

    #[test]
    fn part2_works() {
        let actual = create("test3.txt").solve_part2();

        assert_eq!(&actual, "6");
    }

    #[test]
    fn problem_parse_works() {
        let actual = Problem::parse(&utils::read_file("day08", "test.txt"));

        assert_eq!(actual.instructions, vec!('R', 'L'));
    }
}
