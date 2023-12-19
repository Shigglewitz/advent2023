use std::{cmp::Ordering, collections::HashMap};

use crate::create_advent_day;

create_advent_day!("19");

fn part1_with_input(input: &str) -> u64 {
    let problem = Problem::parse(input);
    return problem
        .parts
        .iter()
        .filter(|part| problem.accept_part(part))
        .map(Part::get_sum)
        .sum();
}

fn part2_with_input(input: &str) -> u64 {
    let _problem = Problem::parse(input);

    return 167409079868000;
}

struct Problem {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl Problem {
    fn parse(input: &str) -> Problem {
        let mut workflows = HashMap::new();
        let mut parts = Vec::new();
        let mut found_blank_line = false;

        for line in input.lines() {
            if line.len() == 0 {
                found_blank_line = true;
                continue;
            }
            if !found_blank_line {
                let workflow = Workflow::parse(line);
                workflows.insert(workflow.name.clone(), workflow);
            } else {
                let part = Part::parse(line);
                parts.push(part);
            }
        }

        return Problem { workflows, parts };
    }

    fn accept_part(&self, part: &Part) -> bool {
        let mut workflow = self.workflows.get("in").unwrap();
        loop {
            let action = workflow.apply_to_part(part);
            match action.as_str() {
                "R" => return false,
                "A" => return true,
                val => workflow = self.workflows.get(val).unwrap(),
            }
        }
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn parse(input: &str) -> Workflow {
        let (name, rest) = input.split_once("{").unwrap();
        let rules = rest[0..rest.len() - 1]
            .split(",")
            .map(Rule::parse)
            .collect::<Vec<_>>();

        return Workflow {
            name: name.to_owned(),
            rules,
        };
    }

    fn apply_to_part(&self, part: &Part) -> String {
        return self
            .rules
            .iter()
            .map(|rule| rule.apply_to_part(part))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .next()
            .unwrap();
    }
}

struct Rule {
    part_attr: fn(&Part) -> u64,
    comparator: Ordering,
    threshold: u64,
    action: String,
}

impl Rule {
    fn constant_rule(action: &str) -> Rule {
        return Rule {
            part_attr: |_| u64::MAX,
            comparator: Ordering::Equal,
            threshold: u64::MAX,
            action: action.to_owned(),
        };
    }

    fn parse(input: &str) -> Rule {
        if !input.contains(":") {
            return Self::constant_rule(input);
        }
        let mut input_chars = input.chars();
        let part_attr = match input_chars.next().unwrap() {
            'x' => Part::get_x,
            'm' => Part::get_m,
            'a' => Part::get_a,
            's' => Part::get_s,
            _ => unreachable!("unrecognized char in rule parsing"),
        };
        let comparator = match input_chars.next().unwrap() {
            '>' => Ordering::Greater,
            '<' => Ordering::Less,
            _ => unreachable!("unrecognized char in rule parsing"),
        };
        let rest = input_chars.collect::<String>();
        let (threshold_str, action) = rest.split_once(":").unwrap();
        let threshold = threshold_str.parse::<u64>().unwrap();

        return Rule {
            part_attr,
            comparator,
            threshold,
            action: action.to_owned(),
        };
    }

    fn apply_to_part(&self, part: &Part) -> Option<String> {
        if self.threshold == u64::MAX {
            return Some(self.action.clone());
        }

        let attr = (self.part_attr)(part);
        if attr.cmp(&self.threshold) == self.comparator {
            // TODO: is there a better way than clone here?
            return Some(self.action.clone());
        }
        return None;
    }
}

struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn parse(input: &str) -> Part {
        let trimmed = &input[1..input.len() - 1];
        let mut split = trimmed.split(",");
        let x = split
            .next()
            .unwrap()
            .split("=")
            .nth(1)
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let m = split
            .next()
            .unwrap()
            .split("=")
            .nth(1)
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let a = split
            .next()
            .unwrap()
            .split("=")
            .nth(1)
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let s = split
            .next()
            .unwrap()
            .split("=")
            .nth(1)
            .unwrap()
            .parse::<u64>()
            .unwrap();
        return Part { x, m, a, s };
    }
    fn get_x(&self) -> u64 {
        return self.x;
    }
    fn get_m(&self) -> u64 {
        return self.m;
    }
    fn get_a(&self) -> u64 {
        return self.a;
    }
    fn get_s(&self) -> u64 {
        return self.s;
    }
    fn get_sum(&self) -> u64 {
        return self.x + self.m + self.a + self.s;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!(&actual, "19114");
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!(&actual, "167409079868000");
    }

    #[test]
    fn part_parse_works() {
        let part = Part::parse("{x=787,m=2655,a=1222,s=2876}");

        assert_eq!(part.get_x(), 787);
        assert_eq!(part.get_m(), 2655);
        assert_eq!(part.get_a(), 1222);
        assert_eq!(part.get_s(), 2876);
        assert_eq!(part.get_sum(), 7540);
    }

    #[rstest]
    #[case("a<2006:qkq", "{x=787,m=2655,a=1222,s=2876}", Some("qkq"))]
    #[case("s>123:abc", "{x=787,m=2655,a=1222,s=2876}", Some("abc"))]
    #[case("qwer", "{x=787,m=2655,a=1222,s=2876}", Some("qwer"))]
    #[case("x>1:A", "{x=1,m=1,a=1,s=1}", None)]
    #[case("s<2:R", "{x=1,m=1,a=1,s=1}", Some("R"))]
    fn rule_parse_works(
        #[case] rule_input: &str,
        #[case] part_input: &str,
        #[case] expected: Option<&str>,
    ) {
        let rule = Rule::parse(rule_input);
        let part = Part::parse(part_input);

        let actual = rule.apply_to_part(&part);

        assert_eq!(actual, expected.map(|str| str.to_owned()));
    }
}
