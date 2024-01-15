use std::{cmp::Ordering, collections::HashMap};

use crate::create_advent_day;

create_advent_day!("2023", "19");

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
    let problem = Problem::parse(input);
    return problem.distinct_combinations_accepted();
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

    fn distinct_combinations_accepted(&self) -> u64 {
        let workflow = self.workflows.get("in").unwrap();
        return workflow.distinct_combinations_accepted(&self.workflows, &mut Vec::new());
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

    fn apply_to_part(&self, part: &Part) -> &String {
        return self
            .rules
            .iter()
            .map(|rule| rule.apply_to_part(part))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .next()
            .unwrap();
    }

    fn distinct_combinations_accepted(
        &self,
        workflow_map: &HashMap<String, Workflow>,
        filters: &mut Vec<RuleComparator>,
    ) -> u64 {
        let mut sum = 0;
        let original_filter_count = filters.len();

        for rule in self.rules.iter() {
            if rule.action == "R" {
                if !rule.is_constant() {
                    filters.push(rule.rule_comparator.flip());
                }
                continue;
            }
            if rule.action != "A" {
                let next_flow = workflow_map.get(&rule.action).unwrap();
                if !rule.is_constant() {
                    filters.push(rule.rule_comparator.clone());
                }
                sum += next_flow.distinct_combinations_accepted(workflow_map, filters);
                if !rule.is_constant() {
                    filters.pop();
                    filters.push(rule.rule_comparator.flip());
                }
                continue;
            }
            if !rule.is_constant() {
                filters.push(rule.rule_comparator.clone());
            }
            let mut x_filters = Vec::new();
            let mut m_filters = Vec::new();
            let mut a_filters = Vec::new();
            let mut s_filters = Vec::new();
            for filter in filters.iter() {
                match filter.part_attr {
                    'x' => x_filters.push(filter),
                    'm' => m_filters.push(filter),
                    'a' => a_filters.push(filter),
                    's' => s_filters.push(filter),
                    _ => unreachable!("unexpected char sorting filters"),
                }
            }

            let xs = Self::calculate_rule_comparators(&mut x_filters);
            let ms = Self::calculate_rule_comparators(&mut m_filters);
            let a_s = Self::calculate_rule_comparators(&mut a_filters);
            let ss = Self::calculate_rule_comparators(&mut s_filters);

            sum += xs * ms * a_s * ss;
            if !rule.is_constant() {
                filters.pop();
                filters.push(rule.rule_comparator.flip());
            }
        }

        while filters.len() > original_filter_count {
            filters.pop();
        }

        return sum;
    }

    fn calculate_rule_comparators(rule_comparators: &mut Vec<&RuleComparator>) -> u64 {
        rule_comparators.insert(
            0,
            &RuleComparator {
                part_attr: 'x',
                comparator: Ordering::Greater,
                threshold: 0,
            },
        );
        rule_comparators.push(&RuleComparator {
            part_attr: 'x',
            comparator: Ordering::Less,
            threshold: 4_001,
        });
        rule_comparators.sort_by(|a, b| a.threshold.cmp(&b.threshold));
        let mut num_comparators = rule_comparators.len();
        let mut index = 0;
        while index < num_comparators - 1 {
            if rule_comparators[index].comparator == rule_comparators[index + 1].comparator {
                if rule_comparators[index].comparator == Ordering::Greater {
                    rule_comparators.remove(index);
                } else {
                    rule_comparators.remove(index + 1);
                }
                num_comparators -= 1;
                continue;
            }
            index += 1;
        }
        return rule_comparators
            .chunks(2)
            .map(|chunk| chunk[1].threshold - chunk[0].threshold - 1)
            .sum::<u64>();
    }
}

struct Rule {
    rule_comparator: RuleComparator,
    action: String,
}

#[derive(Clone, Copy)]
struct RuleComparator {
    part_attr: char,
    comparator: Ordering,
    threshold: u64,
}

impl RuleComparator {
    fn flip(&self) -> RuleComparator {
        let (comparator, threshold) = match self.comparator {
            Ordering::Less => (Ordering::Greater, self.threshold - 1),
            Ordering::Greater => (Ordering::Less, self.threshold + 1),
            _ => unreachable!("should not flip comparators for equals"),
        };
        return RuleComparator {
            part_attr: self.part_attr,
            comparator,
            threshold,
        };
    }
}

impl Rule {
    fn constant_rule(action: &str) -> Rule {
        return Rule {
            rule_comparator: RuleComparator {
                part_attr: ' ',
                comparator: Ordering::Equal,
                threshold: u64::MAX,
            },
            action: action.to_owned(),
        };
    }

    fn is_constant(&self) -> bool {
        return self.rule_comparator.threshold == u64::MAX;
    }

    fn parse(input: &str) -> Rule {
        if !input.contains(":") {
            return Self::constant_rule(input);
        }
        let mut input_chars = input.chars();
        let part_attr = input_chars.next().unwrap();
        let comparator = match input_chars.next().unwrap() {
            '>' => Ordering::Greater,
            '<' => Ordering::Less,
            _ => unreachable!("unrecognized char in rule parsing"),
        };
        let rest = input_chars.collect::<String>();
        let (threshold_str, action) = rest.split_once(":").unwrap();
        let threshold = threshold_str.parse::<u64>().unwrap();

        return Rule {
            rule_comparator: RuleComparator {
                part_attr,
                comparator,
                threshold,
            },
            action: action.to_owned(),
        };
    }

    fn apply_to_part(&self, part: &Part) -> Option<&String> {
        if self.rule_comparator.threshold == u64::MAX {
            return Some(&self.action);
        }

        let attr = match self.rule_comparator.part_attr {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => unreachable!("unrecognized char applying rule to part"),
        };
        if attr.cmp(&self.rule_comparator.threshold) == self.rule_comparator.comparator {
            return Some(&self.action);
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

        assert_eq!(part.x, 787);
        assert_eq!(part.m, 2655);
        assert_eq!(part.a, 1222);
        assert_eq!(part.s, 2876);
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

        assert_eq!(actual, expected.map(|str| str.to_owned()).as_ref());
    }

    #[rstest]
    #[case("in{x>2662:A,R}", (4_000 - 2_662) * 4_000 * 4_000 * 4_000)]
    #[case("in{x<2663:R,A}", (4_000 - 2_662) * 4_000 * 4_000 * 4_000)]
    #[case("in{x>1:A,a>2662:A,R}", (3_999) * 4_000 * 4_000 * 4_000 + 1 * (4_000 - 2_662) * 4_000 * 4_000)]
    #[case("in{x<4000:A,a>2662:A,R}", (3_999) * 4_000 * 4_000 * 4_000 + 1 * (4_000 - 2_662) * 4_000 * 4_000)]
    fn distinct_combinations_with_one_rule(#[case] input: &str, #[case] expected: u64) {
        let problem = Problem::parse(input);
        let actual = problem.distinct_combinations_accepted();

        assert_eq!(actual, expected);
    }
}
