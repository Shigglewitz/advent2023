use crate::create_advent_day;

create_advent_day!("2024", "05");

fn part1_with_input(input: &str) -> i64 {
    let parsed_input = parse_input(input);

    let mut valid_updates = Vec::new();
    for update in parsed_input.updates.iter() {
        let valid = parsed_input.rules.iter().fold(true, |acc: bool, rule| {
            if !acc {
                return acc;
            }
            let mut first_index: i64 = -1;
            let mut second_index: i64 = -1;
            for i in 0..update.len() {
                if update[i] == rule.first {
                    first_index = i as i64;
                    break;
                }
            }
            if first_index == -1 {
                return true;
            }
            for i in 0..update.len() {
                if update[i] == rule.second {
                    second_index = i as i64;
                    break;
                }
            }
            if second_index == -1 {
                return true;
            }
            return first_index < second_index;
        });
        if valid {
            valid_updates.push(update);
        }
    }

    let mut sum = 0;
    for update in valid_updates {
        sum += update[update.len() / 2];
    }

    return sum;
}

fn parse_input(input: &str) -> ParsedInput {
    let mut rules = Vec::new();
    let mut updates = Vec::new();

    let mut found_empty = false;

    for line in input.lines() {
        if line.is_empty() {
            found_empty = true;
            continue;
        }
        if found_empty {
            updates.push(line);
        } else {
            rules.push(line);
        }
    }

    let rules = rules
        .iter()
        .map(|line| {
            let mut split = line.split("|");
            Rule {
                first: split.next().unwrap().parse().unwrap(),
                second: split.next().unwrap().parse().unwrap(),
            }
        })
        .collect::<Vec<Rule>>();
    let updates = updates
        .iter()
        .map(|line| {
            let split = line.split(",");
            split.map(|num| num.parse().unwrap()).collect::<Vec<i64>>()
        })
        .collect();

    ParsedInput { rules, updates }
}

struct Rule {
    first: i64,
    second: i64,
}

struct ParsedInput {
    rules: Vec<Rule>,
    updates: Vec<Vec<i64>>,
}

fn part2_with_input(input: &str) -> i64 {
    let parsed_input = parse_input(input);

    let mut invalid_updates = Vec::new();
    for update in parsed_input.updates.iter() {
        let valid = parsed_input.rules.iter().fold(true, |acc: bool, rule| {
            if !acc {
                return acc;
            }
            let mut first_index: i64 = -1;
            let mut second_index: i64 = -1;
            for i in 0..update.len() {
                if update[i] == rule.first {
                    first_index = i as i64;
                    break;
                }
            }
            if first_index == -1 {
                return true;
            }
            for i in 0..update.len() {
                if update[i] == rule.second {
                    second_index = i as i64;
                    break;
                }
            }
            if second_index == -1 {
                return true;
            }
            return first_index < second_index;
        });
        if !valid {
            invalid_updates.push(update.clone());
        }
    }

    let invalid_updates: Vec<&mut Vec<i64>> = invalid_updates
        .iter_mut()
        .map(|update| {
            let mut clean = false;
            while !clean {
                clean = true;
                for rule in parsed_input.rules.iter() {
                    let mut first_index: i64 = -1;
                    let mut second_index: i64 = -1;
                    for i in 0..update.len() {
                        if update[i] == rule.first {
                            first_index = i as i64;
                            break;
                        }
                    }
                    if first_index == -1 {
                        continue;
                    }
                    for i in 0..update.len() {
                        if update[i] == rule.second {
                            second_index = i as i64;
                            break;
                        }
                    }
                    if second_index == -1 {
                        continue;
                    }
                    if first_index > second_index {
                        clean = false;
                        let temp = update[first_index as usize];
                        update[first_index as usize] = update[second_index as usize];
                        update[second_index as usize] = temp;
                        break;
                    }
                }
            }
            update
        })
        .collect();

    let mut sum = 0;
    for update in invalid_updates {
        sum += update[update.len() / 2];
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = create("test.txt").solve_part1();

        assert_eq!("143", &actual);
    }

    #[test]
    fn part2_works() {
        let actual = create("test.txt").solve_part2();

        assert_eq!("123", &actual);
    }
}
